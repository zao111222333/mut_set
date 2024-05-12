use std::collections::HashSet;

use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::visit_mut::{self, VisitMut};
use syn::{
    parse_quote, token, Attribute, Data, DeriveInput, Error, Expr, Field, Fields, Ident,
    Path, Result, Token, Visibility,
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
    if repr_vec.len() == 0 {
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
    let mut id_func_fields = quote!();
    let mut id_hash = quote!();
    let mut into_fields = quote!();
    let mut partial_cmp = quote!();
    let mut cmp = quote!();
    let mut partial_eq = quote!();
    if indices.is_empty() {
        return Err(Error::new(call_site, "at least specify one `#[id]`"));
    }
    for (i, f) in readonly_fields.iter_mut().enumerate() {
        f.attrs = attr_filter_fn(&f.attrs);
        if indices.contains(&i) {
            f.vis = Visibility::Inherited;
        } else {
            f.vis = to_super(&f.vis);
        }
    }

    let (_id_fields, _other_fields) = rearrange_fields(input_fields, &indices);
    id_fields.clear();
    for f in _id_fields.iter() {
        let t = f.ty.clone();
        let i = f.ident.clone();
        id_hash = quote! {
            #id_hash
            Hash::hash(&self.#i, state);
        };
        id_func_input = quote! {#i: #t, #id_func_input};
        id_func_fields = quote! {#i, #id_func_fields};
        into_fields = quote! {#i:value.#i, #into_fields};
    }
    if sort {
        let i = _id_fields[0].ident.clone();
        partial_eq = quote! {#partial_eq
            self.#i == other.#i
        };
        for f in _id_fields.iter().skip(1) {
            let i = f.ident.clone();
            partial_eq = quote! {#partial_eq
                && self.#i == other.#i
            };
        }
        for f in _id_fields.iter().take(_id_fields.len() - 1) {
            let i = f.ident.clone();
            partial_cmp = quote! {#partial_cmp
                match self.#i.partial_cmp(&other.#i) {
                    Some(core::cmp::Ordering::Equal) => {}
                    ord => return ord,
                }
            };
            cmp = quote! {#cmp
                match self.#i.cmp(&other.#i) {
                    core::cmp::Ordering::Equal => {}
                    ord => return ord,
                }
            };
        }
        let i = _id_fields[_id_fields.len() - 1].ident.clone();
        partial_cmp = quote! {#partial_cmp
            self.#i.partial_cmp(&other.#i)
        };
        cmp = quote! {#cmp
            self.#i.cmp(&other.#i)
        };
    }

    for mut f in _id_fields.into_iter().rev() {
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
    let self_path: Path = parse_quote!(#ident #ty_generics);
    for field in readonly_fields {
        ReplaceSelf::new(&self_path).visit_type_mut(&mut field.ty);
    }

    readonly.ident = Ident::new(&format!("ImmutId{}", input.ident), call_site);
    id.ident = Ident::new(&format!("{}Id", input.ident), call_site);
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
            impl #impl_generics PartialOrd for #ident #ty_generics #where_clause {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    #partial_cmp
                }
            }
            #[doc(hidden)]
            impl #impl_generics Ord for #ident #ty_generics #where_clause {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    #cmp
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
            use std::{borrow::Borrow, hash::{Hash,Hasher}, ops::Deref};
            #id
            impl #impl_generics Hash for #id_ident #ty_generics #where_clause {
                #[inline]
                fn hash<H: Hasher>(&self, state: &mut H) {
                    #id_hash
                }
            }

            #readonly
            impl #impl_generics #ident #ty_generics #where_clause {
                #[inline]
                #readonly_vis fn new_id(#id_func_input)->#id_ident #ty_generics { #id_ident #id_func }
                #[inline]
                #readonly_vis fn id(&self)-> &#id_ident #ty_generics { self.borrow() }
            }

            #sort_quote

            #[doc(hidden)]
            impl #impl_generics Borrow<#id_ident #ty_generics> for #ident #ty_generics #where_clause {
                #[inline]
                fn borrow(&self) -> &#id_ident #ty_generics {
                    unsafe { &*(self as *const Self as *const #id_ident #ty_generics ) }
                }
            }
            impl #impl_generics Hash for #ident #ty_generics #where_clause {
                #[inline]
                fn hash<H: Hasher>(&self, state: &mut H) {
                    <#ident #ty_generics as Borrow<#id_ident #ty_generics>>::borrow(self).hash(state);
                }
            }
            impl #impl_generics mut_set::Item for #ident #ty_generics #where_clause {
                    type ImmutIdItem = #readonly_ident #ty_generics;
                    type Id = #id_ident #ty_generics;
                }
            impl #impl_generics Deref for #readonly_ident #ty_generics #where_clause {
                type Target = #ident #ty_generics;
                #[inline]
                fn deref(&self) -> &Self::Target {
                    unsafe { &*(self as *const Self as *const Self::Target) }
                }
            }
            impl #impl_generics From<#ident #ty_generics> for #readonly_ident #ty_generics #where_clause {
                #[inline]
                fn from(value: #ident #ty_generics) -> Self {
                    Self{#into_fields}
                }
            }
            impl #impl_generics From<#readonly_ident #ty_generics> for #ident #ty_generics #where_clause {
                #[inline]
                fn from(value: #readonly_ident #ty_generics) -> Self {
                    Self{#into_fields}
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
) -> Vec<usize> {
    let mut indices = Vec::new();

    for (i, field) in fields_of_input(input).iter_mut().enumerate() {
        let mut readonly_attr_index = None;

        for (j, attr) in field.attrs.iter().enumerate() {
            if attr.path().is_ident("id") {
                if let Err(err) = attr.meta.require_path_only() {
                    errors.push(err);
                }
                readonly_attr_index = Some(j);
                break;
            }
        }

        if let Some(readonly_attr_index) = readonly_attr_index {
            field.attrs.remove(readonly_attr_index);
            indices.push(i);
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
    indices: &Vec<usize>,
) -> (Vec<Field>, Vec<Field>) {
    let mut in_indices = Vec::new();
    let mut notin_indices = Vec::new();
    let mut i = input_fields.len();
    while let Some(p) = input_fields.pop() {
        i -= 1;
        match p {
            syn::punctuated::Pair::Punctuated(f, _) => {
                if indices.contains(&i) {
                    in_indices.push(f)
                } else {
                    notin_indices.push(f)
                }
            }
            syn::punctuated::Pair::End(_) => todo!(),
        }
    }
    for f in in_indices.iter().rev() {
        let mut _f = f.clone();
        input_fields.push(_f);
    }
    for f in notin_indices.iter().rev() {
        let mut _f = f.clone();
        input_fields.push(_f);
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
                if term != "" {
                    set.insert(term.to_owned());
                }
            }
            Ok(())
        } else {
            return Err(Error::new(call_site, format!("Need terms after `{t}`")));
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
