use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::parse::Parse;
use syn::spanned::Spanned;
use syn::visit_mut::{self, VisitMut};
use syn::{
    parse_quote, parse_str, token, Data, DeriveInput, Error, Expr, Field, Fields, Ident,
    Lit, Path, Result, Token, Visibility,
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
    let id_idx_field_type = rearange_layout_find_id(&mut input, &mut attr_errors);
    input.attrs.push(parse_quote!(#[derive(mut_set::derive::Dummy)]));
    let mut readonly = input.clone();
    readonly.attrs.clear();
    readonly.attrs.push(parse_quote!(#[doc(hidden)]));
    let sort = parser_args(args)?;
    let repr_vec = has_defined_repr(&input);
    if repr_vec.is_empty() {
        input.attrs.push(parse_quote!(#[repr(C)]));
        readonly.attrs.push(parse_quote!(#[repr(C)]));
    } else {
        for attr in repr_vec {
            readonly.attrs.push(attr.clone());
        }
    }
    readonly.vis = to_super(&input.vis);
    let readonly_vis = readonly.vis.clone();
    let readonly_fields = fields_of_input(&mut readonly);
    let mut id_func_input = quote!();
    let mut id_borrow_input = quote!();
    let mut id_hash_func_input = quote!();
    let mut id_input = quote!();
    let mut borrow_check = quote!();
    let mut hash_impl = quote!();
    let mut id_hash_impl = quote!();
    let mut partial_cmp = quote!();
    let mut partial_eq = quote!();
    if id_idx_field_type.is_empty() {
        return Err(Error::new(call_site, "at least specify one `#[id]`"));
    }
    for (i, f) in readonly_fields.iter_mut().enumerate() {
        f.attrs.clear();
        if id_idx_field_type.iter().any(|(idx, _, _)| idx == &i) {
            f.vis = Visibility::Inherited;
        } else {
            f.vis = to_super(&f.vis);
        }
    }
    for (_, f, borrow_type) in id_idx_field_type.iter() {
        let t = f.ty.clone();
        let i = f.ident.clone();
        hash_impl = quote! {
            Hash::hash(&self.#i, &mut state);
            #hash_impl
        };
        id_hash_impl = quote! {
            Hash::hash(&#i, &mut state);
            #id_hash_impl
        };
        id_func_input = quote! {#i, #id_func_input};
        id_input = quote!(#i: #t, #id_input);
        if let Some(borrow_t) = &borrow_type.borrow_type {
            let fn_name =
                Ident::new(&format!("check_fn_{}", i.to_token_stream()), call_site);
            borrow_check = if let Some(check_fn) = &borrow_type.check_fn {
                quote! {
                    fn #fn_name(id: &#t) -> #borrow_t { #check_fn(id) }
                    #borrow_check
                }
            } else {
                quote! {
                    fn #fn_name(id: &#t) -> #borrow_t { id.borrow() }
                    #borrow_check
                }
            };
            let leading_ref = borrow_t.to_string().starts_with("&")
                || borrow_t.to_string().starts_with("Option <&");
            id_hash_func_input = if leading_ref {
                quote! {#i: #borrow_t, #id_hash_func_input}
            } else {
                quote! {#i: &#borrow_t, #id_hash_func_input}
            };
            id_borrow_input = quote!(#fn_name(&#i), #id_borrow_input)
        } else {
            id_hash_func_input = quote! {#i: &#t, #id_hash_func_input};
            id_borrow_input = quote!(&#i, #id_borrow_input)
        }
    }
    if sort {
        let i = id_idx_field_type[0].1.ident.clone();
        partial_eq = quote! {#partial_eq
            self.#i == other.#i
        };
        for (_, f, _) in id_idx_field_type.iter().skip(1) {
            let i = f.ident.clone();
            partial_eq = quote! {#partial_eq
                && self.#i == other.#i
            };
        }
        for (_, f, _) in id_idx_field_type.iter().skip(1) {
            let i = f.ident.clone();
            partial_cmp = quote! {
                match self.#i.partial_cmp(&other.#i) {
                    Some(core::cmp::Ordering::Equal)|None => {}
                    ord => return ord,
                }
                #partial_cmp
            };
        }
        let i = id_idx_field_type[0].1.ident.clone();
        partial_cmp = quote! {#partial_cmp
            self.#i.partial_cmp(&other.#i)
        };
    }
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let mut hash_impl_generics: syn::Generics = parse_quote!(#impl_generics);
    let mut hash_ty_generics: syn::Generics = parse_quote!(#ty_generics);
    if hash_impl_generics.params.is_empty() {
        hash_impl_generics = parse_quote!(<S: BuildHasher + Default>);
    } else {
        hash_impl_generics
            .params
            .insert(0, parse_quote!(S: BuildHasher + Default));
    }
    if hash_ty_generics.params.is_empty() {
        hash_ty_generics = parse_quote!(<S>);
    } else {
        hash_ty_generics.params.insert(0, parse_quote!(S));
    }
    let mut life_hash_impl_generics = hash_impl_generics.clone();
    life_hash_impl_generics.params.insert(0, parse_quote!('a));
    let self_path: Path = parse_quote!(#ident #ty_generics);
    for field in readonly_fields {
        ReplaceSelf::new(&self_path).visit_type_mut(&mut field.ty);
    }

    readonly.ident = Ident::new(&format!("ImmutId{}", input.ident), call_site);
    let id_hash_ident = Ident::new(&format!("{}Id", input.ident), call_site);
    let readonly_ident = &readonly.ident;
    let mut_set_ident = Ident::new(&format!("MutSet{}", input.ident), call_site);
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
        #input
        #[doc(hidden)]
        #[expect(clippy::field_scoped_visibility_modifiers)]
        mod #mod_name {
            #[expect(clippy::wildcard_imports)]
            use super::*;
            use core::{
                borrow::Borrow,
                hash::{Hash,Hasher,BuildHasher},
                ops::{Deref, DerefMut},
            };
            #borrow_check
            #readonly
            #[allow(clippy::ref_option_ref)]
            impl #impl_generics #ident #ty_generics #where_clause {
                #[inline]
                #readonly_vis fn new_id<S: BuildHasher>(
                    __set: &mut_set::MutSet<#ident #ty_generics, S>,
                    #id_hash_func_input
                )-><#ident #ty_generics as mut_set::Item>::Id{
                    let mut state = __set.hasher().build_hasher();
                    #id_hash_impl
                    #id_hash_ident(state.finish())
                }
            }
            #sort_quote
            #readonly_vis struct #id_hash_ident(u64);
            impl core::borrow::Borrow<u64> for #id_hash_ident{
                #[inline]
                fn borrow(&self) -> &u64 {
                    &self.0
                }
            }
            impl From<u64> for #id_hash_ident {
                #[inline]
                fn from(value: u64) -> Self {
                    Self(value)
                }
            }
            #[derive(Debug, Clone, Default)]
            #readonly_vis struct #mut_set_ident #hash_impl_generics(mut_set::MutSet<#ident #ty_generics, S>) #where_clause;
            impl #hash_impl_generics #mut_set_ident #hash_ty_generics #where_clause {
                #[inline]
                #readonly_vis fn serialize_with<Se: serde::Serializer>(
                &self,
                serializer: Se,
                ) -> Result<Se::Ok, Se::Error> {
                    use serde::Serialize;
                    let mut_set: &mut_set::MutSet<_, _> = &self;
                    mut_set.serialize(serializer)
                }
                #[inline]
                #readonly_vis fn deserialize_with<'de, De: serde::Deserializer<'de>>(
                deserializer: De,
                ) -> Result<Self, De::Error> {
                    let mut_set: mut_set::MutSet<#ident #ty_generics, S> = serde::Deserialize::deserialize(deserializer)?;
                    Ok(mut_set.into())
                }
                #[inline]
                #readonly_vis fn contains(
                    &self, #id_hash_func_input
                ) -> bool {
                    let __id = #ident::new_id(&self, #id_func_input);
                    self.id_contains(&__id)
                }
                #[inline]
                #readonly_vis fn get(
                    &self, #id_hash_func_input
                ) -> Option<&#ident #ty_generics> {
                    let __id = #ident::new_id(&self, #id_func_input);
                    self.id_get(&__id)
                }
                #[inline]
                #readonly_vis fn get_mut(
                    &mut self, #id_hash_func_input
                ) -> Option<&mut #readonly_ident #ty_generics> {
                    let __id = #ident::new_id(&self, #id_func_input);
                    self.id_get_mut(&__id)
                }
                #[inline]
                #readonly_vis fn remove(
                    &mut self, #id_hash_func_input
                ) -> bool {
                    let __id = #ident::new_id(&self, #id_func_input);
                    self.id_remove(&__id)
                }
                #[inline]
                #readonly_vis fn take(
                    &mut self, #id_hash_func_input
                ) -> Option<#ident #ty_generics> {
                    let __id = #ident::new_id(&self, #id_func_input);
                    self.id_take(&__id)
                }
                #[inline]
                #readonly_vis fn entry(
                    &mut self, #id_input
                ) -> mut_set::Entry<'_, #ident #ty_generics, impl FnOnce() -> #ident #ty_generics > {
                    let __id = #ident::new_id(&self, #id_borrow_input);
                    self.id_entry(&__id, move || #ident { #id_func_input ..Default::default() })
                }
            }
            impl #hash_impl_generics Deref for #mut_set_ident #hash_ty_generics #where_clause {
                type Target = mut_set::MutSet<#ident #ty_generics, S>;
                #[inline]
                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }
            impl #hash_impl_generics DerefMut for #mut_set_ident #hash_ty_generics #where_clause {
                #[inline]
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
            impl #hash_impl_generics From<mut_set::MutSet<#ident #ty_generics, S>> for #mut_set_ident #hash_ty_generics #where_clause {
                #[inline]
                fn from(value: mut_set::MutSet<#ident #ty_generics, S>) -> Self {
                    Self(value)
                }
            }
            impl #hash_impl_generics From<#mut_set_ident #hash_ty_generics> for mut_set::MutSet<#ident #ty_generics, S> #where_clause {
                #[inline]
                fn from(value: #mut_set_ident #hash_ty_generics) -> Self {
                    value.0
                }
            }
            impl #hash_impl_generics IntoIterator for #mut_set_ident #hash_ty_generics #where_clause {
                type Item = #ident #ty_generics;

                type IntoIter = std::collections::hash_map::IntoValues<u64, #ident #ty_generics>;
                #[inline]
                fn into_iter(self) -> Self::IntoIter {
                    self.0.into_iter()
                }
            }
            impl #life_hash_impl_generics IntoIterator for &'a #mut_set_ident #hash_ty_generics #where_clause {
                type Item = &'a #ident #ty_generics;
                type IntoIter = std::collections::hash_map::Values<'a, u64, #ident #ty_generics>;
                #[inline]
                fn into_iter(self) -> Self::IntoIter {
                    (&self.0).into_iter()
                }
            }
            impl #life_hash_impl_generics IntoIterator for &'a mut #mut_set_ident #hash_ty_generics #where_clause {
                type Item = &'a mut #readonly_ident #ty_generics;
                type IntoIter = mut_set::ValuesMut<'a, #ident #ty_generics>;
                #[inline]
                fn into_iter(self) -> Self::IntoIter {
                    (&mut self.0).into_iter()
                }
            }
            impl #impl_generics mut_set::Item for #ident #ty_generics #where_clause {
                type Id = #id_hash_ident;
                type ImmutIdItem = #readonly_ident #ty_generics;
                type MutSet<S: BuildHasher + Default> = #mut_set_ident #hash_ty_generics;

                #[inline]
                fn id<S: BuildHasher>(&self, __set: &mut_set::MutSet<Self, S>) -> Self::Id {
                    let mut state = __set.hasher().build_hasher();
                    #hash_impl
                    #id_hash_ident(state.finish())
                }
            }
            impl #impl_generics From<#ident #ty_generics> for #readonly_ident #ty_generics #where_clause
            {
                #[inline]
                fn from(value: #ident #ty_generics) -> Self {
                    use std::mem::ManuallyDrop;
                    use std::ptr;
                    unsafe {
                        let this = ManuallyDrop::new(value);
                        let ptr = &*this as *const #ident #ty_generics as *const #readonly_ident #ty_generics;
                        ptr::read(ptr)
                    }
                    // unsafe { std::mem::transmute(value) }
                }
            }
            impl #impl_generics From<#readonly_ident #ty_generics> for #ident #ty_generics #where_clause
            {
                #[inline]
                fn from(value: #readonly_ident #ty_generics) -> Self {
                    use std::mem::ManuallyDrop;
                    use std::ptr;
                    unsafe {
                        let this = ManuallyDrop::new(value);
                        let ptr = &*this as *const #readonly_ident #ty_generics as *const #ident #ty_generics;
                        ptr::read(ptr)
                    }
                    // unsafe { std::mem::transmute(value) }
                }
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

fn rearange_layout_find_id(
    input: &mut DeriveInput,
    errors: &mut Vec<Error>,
) -> Vec<(usize, Field, BorrowType)> {
    let fields = fields_of_input(input);
    let mut id_idx_field_type = Vec::new();
    for (i, field) in fields.iter_mut().enumerate() {
        'L: for (j, attr) in field.attrs.iter().enumerate() {
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
                id_idx_field_type.push((i, field.clone(), borrow_type));
                break 'L;
            }
        }
    }
    let mut idx_map: HashMap<usize, usize>;
    #[cfg(not(feature = "__dbg_disable_mem_layout"))]
    {
        let mut i_size_field_list: Vec<_> = std::mem::take(fields)
            .into_iter()
            .enumerate()
            .map(|(i, mut field)| {
                let mut size_j = None;
                for (j, attr) in field.attrs.iter().enumerate() {
                    if attr.path().is_ident("size") {
                        match &attr.meta {
                            syn::Meta::List(_) | syn::Meta::Path(_) => errors.push(
                                Error::new(attr.meta.span(), "expected #[size = 123 ]"),
                            ),
                            syn::Meta::NameValue(s) => 'm: {
                                if let syn::Expr::Lit(expr_lit) = &s.value {
                                    if let syn::Lit::Int(lit_int) = &expr_lit.lit {
                                        match lit_int.base10_parse::<usize>() {
                                            Ok(n) => size_j = Some((n, j)),
                                            Err(e) => errors.push(e),
                                        }
                                        break 'm;
                                    }
                                }
                                errors.push(syn::Error::new(
                                    attr.meta.span(),
                                    "Expected integer literal",
                                ))
                            }
                        };
                    }
                }
                (
                    i,
                    size_j.map(|(size, j)| {
                        field.attrs.remove(j);
                        size
                    }),
                    field,
                )
            })
            .collect();
        i_size_field_list.sort_by(|(_, a, _), (_, b, _)| match (a, b) {
            (None, None) => std::cmp::Ordering::Equal,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (Some(_), None) => std::cmp::Ordering::Less,
            (Some(a), Some(b)) => b.cmp(a),
        });
        idx_map = i_size_field_list
            .iter()
            .enumerate()
            .map(|(new_idx, (old_idx, _, _))| (*old_idx, new_idx))
            .collect();
        fields.extend(i_size_field_list.into_iter().map(|(old_idx, _, mut f)| {
            let lit = proc_macro2::Literal::u64_unsuffixed(old_idx as u64);
            f.attrs.push(parse_quote!(#[old_pos = #lit]));
            f
        }));
    }
    id_idx_field_type = id_idx_field_type
        .into_iter()
        .map(|(idx, field, _type)| {
            let i: usize;
            cfg_if::cfg_if! {
                if #[cfg(feature = "__dbg_disable_mem_layout")] {
                    i = idx;
                } else {
                    i = idx_map.remove(&idx).unwrap();
                }
            };
            (i, field, _type)
        })
        .collect();
    id_idx_field_type.reverse();
    id_idx_field_type
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
        Visibility::Inherited => parse_quote!(pub(super)),
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
        Visibility::Public(_) => parse_quote!(pub),
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

fn parser_args(args: TokenStream) -> Result<bool> {
    let call_site = proc_macro2::Span::call_site();
    let mut sort = false;
    let mut i = args.into_iter();
    if let Some(arg) = i.next() {
        match arg.to_string().as_str() {
            "sort" => sort = true,
            _ => {
                return Err(Error::new(
                    call_site,
                    format!("macro arguments only support `sort`, find `{}`", arg),
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
    Ok(sort)
}
#[test]
fn size_type_test() {
    // let attr: Attribute = parse_quote!(#[id]);
    // let attr: Attribute = parse_quote!(#[id(borrow="&[ArcStr]")] );
    let attr: syn::Attribute = parse_quote!(#[size = 2]);
    if attr.path().is_ident("size") {
        match &attr.meta {
            syn::Meta::List(_) | syn::Meta::Path(_) => {
                _ = dbg!(Error::new(attr.meta.span(), "expected #[size = 123 ]"))
            }
            syn::Meta::NameValue(s) => {
                if let syn::Expr::Lit(expr_lit) = &s.value {
                    if let syn::Lit::Int(lit_int) = &expr_lit.lit {
                        _ = dbg!(lit_int.base10_parse::<usize>());
                    } else {
                        dbg!(syn::Error::new(
                            attr.meta.span(),
                            "Expected integer literal"
                        ));
                    }
                }
            }
        };
    }
}
#[test]
fn borrow_type_test() {
    // let attr: Attribute = parse_quote!(#[id]);
    // let attr: Attribute = parse_quote!(#[id(borrow="&[ArcStr]")] );
    // let attr: syn::Attribute = parse_quote!(#[id(borrow="&str")]);
    let attr: syn::Attribute = parse_quote!(#[id(borrow = "Option<&str>", check_fn = "mut_set::check_fn::borrow_option")]);
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
        println!("{:?}", borrow_type.borrow_type.unwrap().to_string());
    }
}

#[derive(Debug, Clone, Default)]
struct BorrowType {
    borrow_type: Option<TokenStream>,
    check_fn: Option<TokenStream>,
}

impl syn::parse::Parse for BorrowType {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
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
fn parser_args_test1() {
    let mut hash_impl_generics: syn::Generics = parse_quote!(<T1,T2>);
    hash_impl_generics.params.insert(0, parse_quote!(S: BuildHasher));
    dbg!(hash_impl_generics.to_token_stream().to_string());
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
