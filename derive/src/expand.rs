use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use syn::visit_mut::{self, VisitMut};
use syn::{
    parse_quote, token, Data, DeriveInput, Error, Expr, Field, Fields, Ident, Path,
    Result, Token, Visibility,
};

type Punctuated = syn::punctuated::Punctuated<Field, Token![,]>;

pub fn readonly(input: DeriveInput) -> Result<TokenStream> {
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
    readonly.attrs.push(parse_quote!(#[doc(hidden)]));
    readonly.attrs.push(parse_quote!(#[derive(Debug)]));
    id.attrs.clear();
    id.attrs.push(parse_quote!(#[doc(hidden)]));
    id.attrs.push(parse_quote!(#[derive(Debug)]));
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
    // if !has_defined_repr(&input) {
    // }
    let input_vis = input.vis.clone();
    let v: Visibility = parse_quote!(pub(super));
    if input_vis.to_token_stream().to_string() == v.to_token_stream().to_string() {
        readonly.vis = parse_quote!(pub(in super::super));
    }
    let readonly_vis = readonly.vis.clone();
    id.vis = readonly_vis.clone();

    let input_fields = fields_of_input(&mut input);
    let readonly_fields = fields_of_input(&mut readonly);
    let id_fields = fields_of_input(&mut id);
    let mut id_func_input = quote!();
    let mut id_func_fields = quote!();
    let mut id_hash = quote!();
    let mut into_fields = quote!();
    if indices.is_empty() {
        for field in input_fields {
            field.vis = Visibility::Inherited;
        }
    } else {
        for &i in &indices {
            readonly_fields[i].vis = Visibility::Inherited;
            if let Visibility::Inherited = input_fields[i].vis {
                input_fields[i].vis = input_vis.clone();
            }
        }

        let (_id_fields, _other_fields) = rearrange_fields(input_fields, &indices, false);
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
        for mut f in _id_fields.into_iter().rev() {
            f.attrs.clear();
            id_fields.push(f);
        }
        for f in _other_fields.into_iter() {
            let i = f.ident;
            into_fields = quote! {#i:value.#i, #into_fields};
        }

        rearrange_fields(readonly_fields, &indices, true);
    }
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
                #readonly_vis fn id(#id_func_input)->#id_ident #ty_generics { #id_ident #id_func}
            }
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
    clear_attrs: bool,
) -> (Vec<Field>, Vec<Field>) {
    let mut in_indices = Vec::new();
    let mut notin_indices = Vec::new();
    let mut i = input_fields.len();
    while let Some(p) = input_fields.pop() {
        i -= 1;
        match p {
            syn::punctuated::Pair::Punctuated(f, c) => {
                if indices.contains(&i) {
                    in_indices.push(f)
                } else {
                    notin_indices.push(f)
                }
            }
            syn::punctuated::Pair::End(f) => todo!(),
        }
    }
    for f in in_indices.iter().rev() {
        let mut _f = f.clone();
        if clear_attrs {
            _f.attrs.clear();
        }
        input_fields.push(_f);
    }
    for f in notin_indices.iter().rev() {
        let mut _f = f.clone();
        if clear_attrs {
            _f.attrs.clear();
        }
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
