use std::collections::HashSet;

use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::parse::Parse;
use syn::visit_mut::{self, VisitMut};
use syn::{
    parse_quote, parse_str, token, Attribute, Data, DeriveInput, Error, Expr, Field,
    Fields, Ident, Lit, Path, Result, Token, Visibility,
};

type Punctuated = syn::punctuated::Punctuated<Field, Token![,]>;

pub fn readonly(args: TokenStream, input: DeriveInput) -> Result<TokenStream> {
    let call_site = Span::call_site();

    match &input.data {
        Data::Struct(data) => {
            if data.fields.iter().count() == 0 {
                return Err(Error::new(call_site, "input must be a struct with fields"));
            }
        }
        Data::Enum(_) | Data::Union(_) => {
            return Err(Error::new(call_site, "input must be a struct"));
        }
    }

    let mut input = input;

    let mut attr_errors = Vec::new();
    let indices = find_and_strip_readonly_attrs(&mut input, &mut attr_errors);

    let original_input = quote! {
        #[cfg(doc)]
        #input
    };
    let mut readonly = input.clone();
    let mut id = input.clone();
    readonly.attrs.clear();
    id.attrs.clear();
    readonly.attrs.push(parse_quote!(#[doc(hidden)]));
    id.attrs.push(parse_quote!(#[doc(hidden)]));
    let (sort, macro_set, attri_set) = parser_args(args)?;
    let attr_filter_fn = |v: &Vec<Attribute>| -> Vec<Attribute> {
        let mut _v = vec![];
        for attr in v.iter() {
            let s = attr.meta.to_token_stream().to_string();
            for filter_s in &attri_set {
                if s.starts_with(filter_s) {
                    _v.push(attr.clone());
                    break;
                }
            }
        }
        _v
    };
    for macro_s in macro_set {
        let m: syn::Meta = syn::parse_str(&macro_s)?;
        readonly.attrs.push(parse_quote!(#[#m]));
        id.attrs.push(parse_quote!(#[#m]));
    }
    let repr_vec = has_defined_repr(&input);
    if repr_vec.is_empty() {
        input.attrs.push(parse_quote!(#[repr(C)]));
        readonly.attrs.push(parse_quote!(#[repr(C)]));
        id.attrs.push(parse_quote!(#[repr(C)]));
    } else {
        for attr in repr_vec {
            readonly.attrs.push(attr.clone());
            id.attrs.push(attr);
        }
    }
    readonly.vis = to_super(&input.vis);
    let readonly_vis = readonly.vis.clone();
    id.vis = readonly.vis.clone();

    let input_fields = fields_of_input(&mut input);
    let readonly_fields = fields_of_input(&mut readonly);
    let id_fields = fields_of_input(&mut id);
    let mut id_func_input = quote!();
    let mut id_hash_func_input = quote!();
    let mut borrow_check = quote!();
    let mut id_func_fields = quote!();
    let mut hash_impl = quote!();
    let mut id_hash_impl = quote!();
    let mut into_fields = quote!();
    let mut partial_cmp = quote!();
    let mut partial_eq = quote!();
    if indices.is_empty() {
        return Err(Error::new(call_site, "at least specify one `#[id]`"));
    }
    for (i, f) in readonly_fields.iter_mut().enumerate() {
        f.attrs = attr_filter_fn(&f.attrs);
        if indices.iter().any(|(idx, _)| idx == &i) {
            // if indices.contains(&i) {
            f.vis = Visibility::Inherited;
        } else {
            f.vis = to_super(&f.vis);
        }
    }

    let (_id_fields, _other_fields) = rearrange_fields(input_fields, &indices);
    id_fields.clear();
    for (f, borrow_type) in _id_fields.iter() {
        let t = f.ty.clone();
        let i = f.ident.clone();
        hash_impl = quote! {
            Hash::hash(&self.#i, state);
            #hash_impl
        };
        id_hash_impl = quote! {
            Hash::hash(#i, &mut state);
            #id_hash_impl
        };
        id_func_input = quote! {#i: #t, #id_func_input};
        if let Some(borrow_t) = &borrow_type.borrow_type {
            borrow_check = if let Some(check_fn) = &borrow_type.check_fn {
                quote! {
                    fn #i(id: &#t) -> #borrow_t { #check_fn(id) }
                    #borrow_check
                }
            } else {
                quote! {
                    fn #i(id: &#t) -> #borrow_t { id.borrow() }
                    #borrow_check
                }
            };
            let mut leading_ref = false;
            if let Some(TokenTree::Punct(p)) = borrow_t.clone().into_iter().next() {
                if p.as_char() == '&' {
                    leading_ref = true;
                }
            }
            id_hash_func_input = if leading_ref {
                quote! {#i: #borrow_t, #id_hash_func_input}
            } else {
                quote! {#i: &#borrow_t, #id_hash_func_input}
            };
        } else {
            id_hash_func_input = quote! {#i: &#t, #id_hash_func_input};
        }
        id_func_fields = quote! {#i, #id_func_fields};
        into_fields = quote! {#i:value.#i, #into_fields};
    }
    if sort {
        let i = _id_fields[0].0.ident.clone();
        partial_eq = quote! {#partial_eq
            self.#i == other.#i
        };
        for (f, _) in _id_fields.iter().skip(1) {
            let i = f.ident.clone();
            partial_eq = quote! {#partial_eq
                && self.#i == other.#i
            };
        }
        for (f, _) in _id_fields.iter().skip(1) {
            let i = f.ident.clone();
            partial_cmp = quote! {
                match self.#i.partial_cmp(&other.#i) {
                    Some(core::cmp::Ordering::Equal)|None => {}
                    ord => return ord,
                }
                #partial_cmp
            };
        }
        let i = _id_fields[0].0.ident.clone();
        partial_cmp = quote! {#partial_cmp
            self.#i.partial_cmp(&other.#i)
        };
    }

    for (mut f, _) in _id_fields.clone().into_iter().rev() {
        f.attrs = attr_filter_fn(&f.attrs);
        f.vis = to_super(&f.vis);
        id_fields.push(f);
    }
    for f in _other_fields.into_iter() {
        let i = f.ident;
        into_fields = quote! {#i:value.#i, #into_fields};
    }

    rearrange_fields(readonly_fields, &indices);
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let _ty_generics: syn::Generics = parse_quote!(#ty_generics);
    let lt_token = &_ty_generics.lt_token;
    let gt_token = &_ty_generics.gt_token;
    let params = &_ty_generics.params;
    let phantom_type = quote! {#lt_token (#params)#gt_token};
    let id_func = if ty_generics.to_token_stream().is_empty() {
        quote! {{#id_func_fields} }
    } else {
        id_fields.push(parse_quote!(_p: std::marker::PhantomData #phantom_type ));
        id_func_fields =
            quote! {_p: std::marker::PhantomData :: #phantom_type , #id_func_fields};
        quote! {:: #ty_generics {#id_func_fields} }
    };
    let borrow_when_single_id = if _id_fields.len() == 1 {
        let (f, _) = _id_fields.first().unwrap();
        let t = f.ty.clone();
        let i = f.ident.clone();
        quote! {
            impl #impl_generics Borrow<#t> for #ident #ty_generics #where_clause {
                #[inline]
                fn borrow(&self) -> &#t {
                    &self.#i
                }
            }
        }
    } else {
        quote!()
    };
    let self_path: Path = parse_quote!(#ident #ty_generics);
    for field in readonly_fields {
        ReplaceSelf::new(&self_path).visit_type_mut(&mut field.ty);
    }

    readonly.ident = Ident::new(&format!("ImmutId{}", input.ident), call_site);
    id.ident = Ident::new(&format!("{}Id", input.ident), call_site);
    let id_hash_ident = Ident::new(&format!("{}BorrowId", input.ident), call_site);
    let readonly_ident = &readonly.ident;
    let id_ident = &id.ident;
    let mod_name =
        Ident::new(&format!("__{}", to_snake_case(&ident.to_string())), call_site);

    let attr_errors = attr_errors.iter().map(Error::to_compile_error);
    let sort_quote = if sort {
        quote! {
            #[doc(hidden)]
            impl #impl_generics PartialEq for #ident #ty_generics #where_clause {
                #[inline]
                fn eq(&self, other: &Self) -> bool {
                    #partial_eq
                }
            }
            #[doc(hidden)]
            impl #impl_generics Eq for #ident #ty_generics #where_clause {}
            #[doc(hidden)]
            #[allow(clippy::non_canonical_partial_ord_impl)]
            impl #impl_generics PartialOrd for #ident #ty_generics #where_clause {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    #partial_cmp
                }
            }
            #[doc(hidden)]
            impl #impl_generics Ord for #ident #ty_generics #where_clause {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
                }
            }
        }
    } else {
        quote! {}
    };
    input.attrs.insert(0, parse_quote!(#[cfg(not(doc))]));
    Ok(quote! {
        #original_input

        #input
        #[doc(hidden)]
        mod #mod_name{
            use super::*;
            use std::{borrow::Borrow, hash::{Hash,Hasher}, ops::{Deref}};
            #id
            impl #impl_generics Hash for #id_ident #ty_generics #where_clause {
                #[inline]
                fn hash<H: Hasher>(&self, state: &mut H) {
                    #hash_impl
                }
            }

            #readonly
            #[allow(clippy::ref_option_ref)]
            impl #impl_generics #ident #ty_generics #where_clause {
                #[inline]
                #readonly_vis fn new_id(#id_func_input)->#id_ident #ty_generics { #id_ident #id_func }
                #[inline]
                #readonly_vis fn id(&self)-> &#id_ident #ty_generics { <#ident #ty_generics as Borrow<#id_ident #ty_generics>>::borrow(self) }
                const CHECK: () = {
                    #borrow_check
                };
                #[inline]
                #readonly_vis fn borrow_id(
                    __set: &mut_set::MutSet<#ident #ty_generics>, #id_hash_func_input
                )-><#ident #ty_generics as mut_set::Item>::BorrowId{
                    use std::hash::BuildHasher;
                    let mut state = __set.hasher().build_hasher();
                    #id_hash_impl
                    state.finish().into()
                }
            }
            #sort_quote

            #[doc(hidden)]
            impl #impl_generics Borrow<#id_ident #ty_generics> for #ident #ty_generics #where_clause {
                #[inline]
                fn borrow(&self) -> &#id_ident #ty_generics {
                    unsafe { &*(self as *const Self as *const #id_ident #ty_generics ) }
                }
            }
            #borrow_when_single_id
            impl #impl_generics Hash for #ident #ty_generics #where_clause {
                #[inline]
                fn hash<H: Hasher>(&self, state: &mut H) {
                    <#ident #ty_generics as Borrow<#id_ident #ty_generics>>::borrow(self).hash(state);
                }
            }
            #readonly_vis struct #id_hash_ident(u64);
            impl From<u64> for #id_hash_ident {
                #[inline]
                fn from(value: u64) -> Self { Self(value) }
            }
            impl Into<u64> for #id_hash_ident {
                #[inline]
                fn into(self) -> u64 { self.0 }
            }
            impl #impl_generics mut_set::Item for #ident #ty_generics #where_clause {
                type Id = #id_ident #ty_generics;
                type BorrowId = #id_hash_ident;
                type ImmutIdItem = #readonly_ident #ty_generics;
            }
            impl #impl_generics Deref for #readonly_ident #ty_generics #where_clause {
                type Target = #ident #ty_generics;
                #[inline]
                fn deref(&self) -> &Self::Target {
                    unsafe { &*(self as *const Self as *const Self::Target) }
                }
            }
            impl #impl_generics mut_set::MutSetDeref for #ident #ty_generics #where_clause {
                type Target = #readonly_ident #ty_generics;
                #[inline]
                fn mut_set_deref(&mut self) -> &mut Self::Target {
                    unsafe { &mut *(self as *mut Self as *mut Self::Target) }
                }
            }
        }
        #(#attr_errors)*
    })
}

// TODO
fn has_defined_repr(input: &DeriveInput) -> Vec<syn::Attribute> {
    let mut repr_vec = vec![];
    for attr in &input.attrs {
        if !attr.path().is_ident("repr") {
            continue;
        }
        let _ = attr.parse_nested_meta(|meta| {
            let path = &meta.path;
            if path.is_ident("C")
                || path.is_ident("transparent")
                || path.is_ident("packed")
            {
                repr_vec.push(attr.clone());
            }
            if meta.input.peek(Token![=]) {
                let _value: Expr = meta.value()?.parse()?;
            } else if meta.input.peek(token::Paren) {
                let _group: TokenTree = meta.input.parse()?;
            }
            Ok(())
        });
    }
    repr_vec
}

fn fields_of_input(input: &mut DeriveInput) -> &mut Punctuated {
    match &mut input.data {
        Data::Struct(data) => match &mut data.fields {
            Fields::Named(fields) => &mut fields.named,
            Fields::Unnamed(fields) => &mut fields.unnamed,
            Fields::Unit => unreachable!(),
        },
        Data::Enum(_) | Data::Union(_) => unreachable!(),
    }
}

fn find_and_strip_readonly_attrs(
    input: &mut DeriveInput,
    errors: &mut Vec<Error>,
) -> Vec<(usize, BorrowType)> {
    let mut indices = Vec::new();

    for (i, field) in fields_of_input(input).iter_mut().enumerate() {
        for (j, attr) in field.attrs.iter().enumerate() {
            if attr.path().is_ident("id") {
                let borrow_type = match attr.meta {
                    syn::Meta::Path(_) => BorrowType::default(),
                    syn::Meta::List(_) => match attr.parse_args_with(BorrowType::parse) {
                        Ok(t) => t,
                        Err(e) => {
                            errors.push(e);
                            BorrowType::default()
                        }
                    },
                    syn::Meta::NameValue(_) => todo!(),
                };
                field.attrs.remove(j);
                indices.push((i, borrow_type));
                break;
            }
        }
    }
    indices
}

struct ReplaceSelf<'a> {
    with: &'a Path,
}

impl<'a> ReplaceSelf<'a> {
    fn new(with: &'a Path) -> Self {
        ReplaceSelf { with }
    }
}

impl<'a> VisitMut for ReplaceSelf<'a> {
    fn visit_path_mut(&mut self, path: &mut Path) {
        if path.is_ident("Self") {
            let span = path.segments[0].ident.span();
            *path = self.with.clone();
            path.segments[0].ident.set_span(span);
        } else {
            visit_mut::visit_path_mut(self, path);
        }
    }
}

fn rearrange_fields(
    input_fields: &mut Punctuated,
    indices: &[(usize, BorrowType)],
) -> (Vec<(Field, BorrowType)>, Vec<Field>) {
    let mut in_indices = Vec::new();
    let mut notin_indices = Vec::new();
    let mut i = input_fields.len();
    while let Some(p) = input_fields.pop() {
        i -= 1;
        match p {
            syn::punctuated::Pair::Punctuated(f, _) | syn::punctuated::Pair::End(f) => {
                let mut find = false;
                'L: for (idx, borrow_type) in indices {
                    if &i == idx {
                        find = true;
                        in_indices.push((f.clone(), borrow_type.clone()));
                        break 'L;
                    }
                }
                if !find {
                    notin_indices.push(f);
                }
                // if indices.contains(&i) {
                //     in_indices.push(f)
                // } else {
                //     notin_indices.push(f)
                // }
            }
        }
    }
    for (f, _) in in_indices.iter().rev() {
        input_fields.push(f.clone());
    }
    for f in notin_indices.iter().rev() {
        input_fields.push(f.clone());
    }
    (in_indices, notin_indices)
}

fn to_snake_case(s: &str) -> String {
    let mut snake_case = String::new();
    let chars: Vec<char> = s.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        if c.is_uppercase() && i != 0 {
            snake_case.push('_');
        }
        snake_case.push(c.to_lowercase().next().unwrap());
    }
    snake_case
}
fn to_super(vis: &Visibility) -> Visibility {
    match vis {
        Visibility::Restricted(vis_r) => {
            let path = vis_r.path.to_token_stream().to_string();
            if path.starts_with("crate") {
                vis.clone()
            } else {
                let mut _vis_r = vis_r.clone();
                match path.as_str() {
                    "self" => {
                        _vis_r.path = parse_quote!(super);
                        Visibility::Restricted(_vis_r)
                    }
                    "super" => {
                        parse_quote!(pub(in super::super))
                    }
                    _ => {
                        let path = _vis_r.path;
                        _vis_r.path = parse_quote!(super::#path);
                        Visibility::Restricted(_vis_r)
                    }
                }
            }
        }
        _ => vis.clone(),
    }
}

#[test]
fn vis_sup() {
    fn cmp(origin: Visibility, sup: Visibility) {
        assert_eq!(
            to_super(&origin).to_token_stream().to_string(),
            sup.to_token_stream().to_string()
        );
    }
    cmp(parse_quote!(pub(self)), parse_quote!(pub(super)));
    cmp(parse_quote!(pub(super)), parse_quote!(pub(in super::super)));
    cmp(parse_quote!(pub(crate)), parse_quote!(pub(crate)));
    cmp(
        parse_quote!(pub(in super::self::super)),
        parse_quote!(pub(in super::super::self::super)),
    );
    cmp(parse_quote!(pub(in crate::mod_a)), parse_quote!(pub(in crate::mod_a)));
}

fn parser_args(args: TokenStream) -> Result<(bool, HashSet<String>, HashSet<String>)> {
    let call_site = proc_macro2::Span::call_site();
    let mut sort = false;
    let mut macro_set = HashSet::new();
    let mut attri_set = HashSet::new();

    fn process_one(
        i: &mut proc_macro2::token_stream::IntoIter,
        t: &str,
        set: &mut HashSet<String>,
    ) -> syn::Result<()> {
        let call_site = Span::call_site();
        if let Some(arg) = i.next() {
            let s = arg.to_string();
            let mut chars = s.chars();
            if let Some('(') = chars.next() {
            } else {
                return Err(Error::new(
                    call_site,
                    format!("`{t}` should be surrounded by paren `(` and `)`"),
                ));
            }
            if let Some(')') = chars.next_back() {
            } else {
                return Err(Error::new(
                    call_site,
                    format!("`{t}` should be surrounded by paren `(` and `)`"),
                ));
            }
            for term in chars.as_str().to_string().replace(" ", "").split(';') {
                if !term.is_empty() {
                    set.insert(term.to_owned());
                }
            }
            Ok(())
        } else {
            Err(Error::new(call_site, format!("Need terms after `{t}`")))
        }
    }
    let mut i = args.into_iter();
    if let Some(arg) = i.next() {
        match arg.to_string().as_str() {
            "sort" => sort = true,
            "macro" => {
                process_one(&mut i, "macro", &mut macro_set)?;
            }
            "attr_filter" => {
                process_one(&mut i, "attri", &mut attri_set)?;
            }
            _ => {
                return Err(Error::new(
                    call_site,
                    format!(
                    "macro arguments only support `macro` and `attr_filter`, find `{}`",
                    arg
                ),
                ))
            }
        }
    }
    if let Some(arg) = i.next() {
        match arg.to_string().as_str() {
            "," => (),
            _ => return Err(Error::new(call_site, format!("want `,` find `{}`", arg))),
        }
    }
    if let Some(arg) = i.next() {
        match arg.to_string().as_str() {
            "sort" => sort = true,
            "macro" => {
                process_one(&mut i, "macro", &mut macro_set)?;
            }
            "attr_filter" => {
                process_one(&mut i, "attr_filter", &mut attri_set)?;
            }
            _ => {
                return Err(Error::new(
                    call_site,
                    format!(
                    "macro arguments only support `macro` and `attr_filter`, find `{}`",
                    arg
                ),
                ))
            }
        }
    }
    if let Some(arg) = i.next() {
        match arg.to_string().as_str() {
            "," => (),
            _ => return Err(Error::new(call_site, format!("want `,` find `{}`", arg))),
        }
    }
    if let Some(arg) = i.next() {
        match arg.to_string().as_str() {
            "sort" => sort = true,
            "macro" => {
                process_one(&mut i, "macro", &mut macro_set)?;
            }
            "attr_filter" => {
                process_one(&mut i, "attr_filter", &mut attri_set)?;
            }
            _ => {
                return Err(Error::new(
                    call_site,
                    format!(
                    "macro arguments only support `macro` and `attr_filter`, find `{}`",
                    arg
                ),
                ))
            }
        }
    }
    if let Some(arg) = i.next() {
        return Err(Error::new(call_site, format!("want nothing, find `{}`", arg)));
    }
    Ok((sort, macro_set, attri_set))
}
#[test]
fn borrow_type_test() {
    // let attr: Attribute = parse_quote!(#[id]);
    // let attr: Attribute = parse_quote!(#[id(borrow="&[ArcStr]")] );
    let attr: Attribute = parse_quote!(#[id(borrow="&str")]);
    // let attr: Attribute = parse_quote!(#[id(borrow = "Option<&str>", check_fn = "mut_set::check_fn::borrow_option")]);
    if attr.path().is_ident("id") {
        let borrow_type = match attr.meta {
            syn::Meta::Path(_) => BorrowType::default(),
            syn::Meta::List(_) => match attr.parse_args_with(BorrowType::parse) {
                Ok(t) => t,
                Err(e) => {
                    println!("{e:?}");
                    BorrowType::default()
                }
            },
            syn::Meta::NameValue(_) => todo!(),
        };
        println!("{:?}", borrow_type.borrow_type);
    }
}

#[derive(Debug, Clone, Default)]
struct BorrowType {
    borrow_type: Option<TokenStream>,
    check_fn: Option<TokenStream>,
}

impl syn::parse::Parse for BorrowType {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        // let mut borrow_type = None;
        // if input.peek(syn::Ident) && input.peek2(Token![=]) {
        //     let ident: syn::Ident = input.parse()?;
        //     if ident == "borrow" {
        //         let _: Token![=] = input.parse()?;
        //         let lit: Lit = input.parse()?;
        //         if let Lit::Str(lit_str) = lit {
        //             borrow_type = Some(parse_str(&lit_str.value())?);
        //         }
        //     }
        // }
        // Ok(Self(borrow_type))
        let mut borrow_type = None;
        let mut check_fn = None;
        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            let _: Token![=] = input.parse()?;
            let lit: Lit = input.parse()?;
            if ident == "borrow" {
                if let Lit::Str(lit_str) = lit {
                    borrow_type = Some(parse_str(&lit_str.value())?);
                }
            } else if ident == "check_fn" {
                if let Lit::Str(lit_str) = lit {
                    check_fn = Some(parse_str(&lit_str.value())?);
                }
            }
            if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
            }
        }
        Ok(Self { borrow_type, check_fn })
    }
}
#[test]
fn parser_args_test() {
    println!(
        "{:?}",
        parser_args(quote! {
        macro ( derive(Debug, Clone);
        derive(derivative::Derivative);
        derivative(Default);),
        attri ( derivative;)
        })
    );
    println!(
        "{:?}",
        parser_args(quote! {
        macro ( derive(Debug, Clone);
        derive(derivative::Derivative);
        derivative(Default);)
        })
    );
    println!("{:?}", parser_args(quote! {}));
}
