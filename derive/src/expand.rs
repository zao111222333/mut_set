use proc_macro2::{Span, TokenStream, TokenTree};
use quote::{ToTokens, quote};
use syn::parse::Parse;
use syn::visit_mut::{self, VisitMut};
use syn::{
    Data, DeriveInput, Error, Expr, Field, Fields, Ident, Path, Result, Token, Type,
    Visibility, parse_quote, token,
};

type Punctuated = syn::punctuated::Punctuated<Field, Token![,]>;

pub fn readonly(mut input: DeriveInput) -> Result<TokenStream> {
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
    let mut attr_errors = Vec::new();
    let id_field_type = rearange_by_id(&mut input, &mut attr_errors);
    let doc = quote! {
        #[cfg(doc)]
        #input
    };
    if id_field_type.is_empty() {
        return Err(Error::new(call_site, "at least specify one `#[id]`"));
    }
    let mut readonly = input.clone();
    let mut id: syn::DeriveInput = parse_quote! {
        struct Id {}
    };
    readonly.attrs.clear();
    readonly.attrs.push(parse_quote!(#[doc(hidden)]));
    id.attrs.clear();
    id.attrs.push(parse_quote!(#[doc(hidden)]));
    if !has_defined_repr(&input).is_empty() {
        return Err(Error::new(call_site, "Should not have `#[repr]`"));
    }
    input.attrs.push(parse_quote!(#[repr(C)]));
    readonly.attrs.push(parse_quote!(#[repr(C)]));
    id.attrs.push(parse_quote!(#[repr(C)]));
    readonly.vis = to_super(&input.vis);
    id.vis = input.vis.clone();
    let readonly_fields = fields_of_input(&mut readonly);
    let id_fields = fields_of_input(&mut id);
    let mut id_func_input = quote!();
    let mut id_borrow_input = quote!();
    let mut new_id_input = quote!();
    let mut new_id_field = quote!();
    let mut id_input = quote!();
    let mut hash_impl = quote!();
    let mut partial_cmp = quote!();
    for (i, f) in readonly_fields.iter_mut().enumerate() {
        f.attrs.clear();
        if i < id_field_type.len() {
            id_fields.push(f.clone());
            f.vis = Visibility::Inherited;
        } else {
            f.vis = to_super(&f.vis);
        }
    }
    for (f, borrow_type) in id_field_type.iter() {
        let t = f.ty.clone();
        let i = f.ident.clone();
        hash_impl = if let Some(into_hash_ord_fn) = &borrow_type.into_hash_ord_fn {
            quote! {
                #hash_impl
                Hash::hash(&#into_hash_ord_fn(&self.#i), state);
            }
        } else {
            quote! {
                #hash_impl
                Hash::hash(&self.#i, state);
            }
        };
        id_func_input = quote! {#i, #id_func_input};
        id_input = quote!(#i: #t, #id_input);
        new_id_input = quote! {#new_id_input #i: #t,};
        new_id_field = quote! {#new_id_field #i,};
        id_borrow_input = quote!(&#i, #id_borrow_input)
    }
    let mut id_field_type_rev = id_field_type.clone();
    id_field_type_rev.reverse();
    let i = id_field_type_rev[0].0.ident.clone();
    let mut partial_eq =
        if let Some(into_hash_ord_fn) = &id_field_type_rev[0].1.into_hash_ord_fn {
            quote! {
                #into_hash_ord_fn(&self.#i) == #into_hash_ord_fn(&other.#i)
            }
        } else {
            quote! {
                self.#i == other.#i
            }
        };
    for (f, borrow_type) in id_field_type_rev.iter().skip(1) {
        let i = f.ident.clone();
        (partial_eq, partial_cmp) = if let Some(into_hash_ord_fn) =
            &borrow_type.into_hash_ord_fn
        {
            (
                quote! { #into_hash_ord_fn(&self.#i) == #into_hash_ord_fn(&other.#i) && #partial_eq },
                quote! {
                    match #into_hash_ord_fn(&self.#i).partial_cmp(&#into_hash_ord_fn(&other.#i)) {
                        Some(core::cmp::Ordering::Equal)|None => {}
                        ord => return ord,
                    }
                    #partial_cmp
                },
            )
        } else {
            (
                quote! { self.#i == other.#i && #partial_eq },
                quote! {
                    match self.#i.partial_cmp(&other.#i) {
                        Some(core::cmp::Ordering::Equal)|None => {}
                        ord => return ord,
                    }
                    #partial_cmp
                },
            )
        };
    }
    let i = id_field_type_rev[0].0.ident.clone();
    partial_cmp = if let Some(into_hash_ord_fn) = &id_field_type_rev[0].1.into_hash_ord_fn
    {
        quote! {#partial_cmp
            #into_hash_ord_fn(&self.#i).partial_cmp(&#into_hash_ord_fn(&other.#i))
        }
    } else {
        quote! {#partial_cmp
            self.#i.partial_cmp(&other.#i)
        }
    };
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let mut hash_impl_generics: syn::Generics = parse_quote!(#impl_generics);
    if hash_impl_generics.params.is_empty() {
        hash_impl_generics = parse_quote!(<S: BuildHasher + Default>);
    } else {
        hash_impl_generics
            .params
            .insert(0, parse_quote!(S: BuildHasher + Default));
    }
    let mut life_hash_impl_generics = hash_impl_generics.clone();
    life_hash_impl_generics.params.insert(0, parse_quote!('a));
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
    input.attrs.insert(0, parse_quote!(#[cfg(not(doc))]));
    let (id_define, id_impls) = if id_field_type.len() == 1
        && id_field_type[0].1.into_hash_ord_fn.is_none()
    {
        let unique_id_ident = id_field_type[0].0.ident.as_ref().unwrap();
        let unique_id_type = &id_field_type[0].0.ty;
        let extra_borrow = if let Some(unique_id_borrow_type) =
            id_field_type[0].1.borrow_type.as_ref()
        {
            quote! {
                impl #impl_generics Borrow<#unique_id_borrow_type> for #ident #ty_generics #where_clause {
                    fn borrow(&self) -> &#unique_id_borrow_type {
                        &self.#unique_id_ident
                    }
                }
            }
        } else {
            quote! {}
        };
        (
            quote! {
                pub type #id_ident = #unique_id_type;
            },
            quote! {
                #extra_borrow
                impl #impl_generics Borrow<#id_ident> for #ident #ty_generics #where_clause {
                    fn borrow(&self) -> &#id_ident {
                        &self.#unique_id_ident
                    }
                }
            },
        )
    } else {
        (
            quote! {
                #id
                impl #id_ident {
                    #[inline]
                    pub fn new(#new_id_input) -> Self { Self{#new_id_field} }
                }
            },
            quote! {
                #[doc(hidden)]
                impl Hash for #id_ident {
                    #[inline]
                    fn hash<H: Hasher>(&self, state: &mut H) {
                        #hash_impl
                    }
                }
                #[doc(hidden)]
                impl PartialEq for #id_ident {
                    #[inline]
                    fn eq(&self, other: &Self) -> bool {
                        #partial_eq
                    }
                }
                #[doc(hidden)]
                impl Eq for #id_ident {}
                #[doc(hidden)]
                #[allow(clippy::non_canonical_partial_ord_impl)]
                impl PartialOrd for #id_ident {
                    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                        #partial_cmp
                    }
                }
                #[doc(hidden)]
                impl Ord for #id_ident {
                    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
                    }
                }
                impl #impl_generics Borrow<#id_ident> for #ident #ty_generics #where_clause {
                    fn borrow(&self) -> &#id_ident {
                        unsafe { &*(self as *const Self as *const #id_ident) }
                    }
                }
            },
        )
    };
    Ok(quote! {
        #doc
        #input
        #id_define
        #[doc(hidden)]
        #[expect(clippy::field_scoped_visibility_modifiers)]
        mod #mod_name {
            #[expect(clippy::wildcard_imports)]
            use super::*;
            use mut_set::Item as _;
            use core::{
                borrow::Borrow,
                hash::{Hash,Hasher},
                ops::Deref,
            };
            #readonly
            #id_impls
            #[doc(hidden)]
            impl #impl_generics Hash for #ident #ty_generics #where_clause {
                #[inline]
                fn hash<H: Hasher>(&self, state: &mut H) {
                    self.id().hash(state)
                }
            }
            #[doc(hidden)]
            impl #impl_generics PartialEq for #ident #ty_generics #where_clause {
                #[inline]
                fn eq(&self, other: &Self) -> bool {
                    self.id().eq(other.id())
                }
            }
            #[doc(hidden)]
            impl #impl_generics Eq for #ident #ty_generics #where_clause {}
            #[doc(hidden)]
            #[allow(clippy::non_canonical_partial_ord_impl)]
            impl #impl_generics PartialOrd for #ident #ty_generics #where_clause {
                fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                    self.id().partial_cmp(other.id())
                }
            }
            #[doc(hidden)]
            impl #impl_generics Ord for #ident #ty_generics #where_clause {
                fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                    self.id().cmp(other.id())
                }
            }
            impl #impl_generics mut_set::Item for #ident #ty_generics #where_clause {
                type Id = #id_ident;
                type ImmutIdItem = #readonly_ident #ty_generics;
                #[expect(invalid_reference_casting)]
                fn __unsafe_deref_mut(&self) -> &mut Self::ImmutIdItem {
                    unsafe { &mut *(self as *const Self as *mut Self::ImmutIdItem) }
                }
            }
            impl #impl_generics Deref for #readonly_ident #ty_generics #where_clause {
                type Target = #ident #ty_generics;
                #[inline]
                fn deref(&self) -> &Self::Target {
                    unsafe { &*(self as *const Self as *const Self::Target) }
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

fn rearange_by_id(
    input: &mut DeriveInput,
    errors: &mut Vec<Error>,
) -> Vec<(Field, BorrowType)> {
    let fields = fields_of_input(input);
    let mut id_field_type = Vec::new();
    let mut id_fields = Punctuated::new();
    let mut other_fields = Punctuated::new();
    'L: for field in fields.iter() {
        let mut field = field.clone();
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
                id_field_type.push((field.clone(), borrow_type));
                id_fields.push(field.clone());
                continue 'L;
            }
        }
        other_fields.push(field.clone());
    }
    fields.clear();
    fields.extend(id_fields);
    fields.extend(other_fields);
    id_field_type
}

struct ReplaceSelf<'a> {
    with: &'a Path,
}

impl<'a> ReplaceSelf<'a> {
    fn new(with: &'a Path) -> Self {
        ReplaceSelf { with }
    }
}

impl VisitMut for ReplaceSelf<'_> {
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

#[test]
fn rearange_test() {
    let mut input: syn::DeriveInput = parse_quote! {
        pub struct MyItem<T1, T2>
        where
            T1: Sized,
        {
            #[id(borrow = [String])]
            pub(self) id1: usize,
            pub(crate) ctx1: T1,
            pub ctx2: T2,
            #[id(into_hash_ord_fn = my_fn)]
            pub id2: String,
        }
    };
    let mut errors = Vec::new();
    let id_field_type = rearange_by_id(&mut input, &mut errors);
    println!("{}", input.into_token_stream().to_string());
    for (field, _type) in id_field_type {
        if let Some(borrow_type) = _type.borrow_type {
            println!("{}", borrow_type.to_token_stream());
        }
        if let Some(expr) = _type.into_hash_ord_fn {
            println!("{}", expr.to_token_stream().to_string());
        }
        println!("{}", field.to_token_stream().to_string());
    }
}

#[derive(Clone, Default)]
struct BorrowType {
    borrow_type: Option<Type>,
    into_hash_ord_fn: Option<Expr>,
}

impl syn::parse::Parse for BorrowType {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let mut borrow_type = None;
        let mut into_hash_ord_fn = None;
        while !input.is_empty() {
            let ident: syn::Ident = input.parse()?;
            let _: Token![=] = input.parse()?;
            match ident.to_string().as_str() {
                "borrow" => {
                    borrow_type = Some(input.parse()?);
                }
                "into_hash_ord_fn" => {
                    into_hash_ord_fn = Some(input.parse()?);
                }
                _ => {}
            }
            if input.peek(Token![,]) {
                let _: Token![,] = input.parse()?;
            }
        }
        Ok(Self { borrow_type, into_hash_ord_fn })
    }
}
#[test]
fn parser_args_test1() {
    let mut hash_impl_generics: syn::Generics = parse_quote!(<T1,T2>);
    hash_impl_generics.params.insert(0, parse_quote!(S: BuildHasher));
    dbg!(hash_impl_generics.to_token_stream().to_string());
}
