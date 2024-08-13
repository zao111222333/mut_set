//! See more at [![github](https://img.shields.io/badge/github-main-blue?logo=github)](https://github.com/zao111222333/mut_set)
//!
//!```
//!#[derive(Debug)]
//!#[mut_set::derive::item]
//!pub struct MyItem<T1, T2>
//!where
//!    T1: Sized,
//!{
//!    #[id]
//!    pub(self) id1: usize,
//!    pub(crate) ctx1: T1,
//!    pub ctx2: T2,
//!    #[id]
//!    pub id2: String,
//!}
//!#[test]
//!fn test() {
//!    let mut set = mut_set::MutSet::new();
//!    println!("{:?}", set);
//!    set.insert(MyItem {
//!        id1: 2,
//!        id2: "www".to_string(),
//!        ctx1: -1,
//!        ctx2: "ccc".to_string(),
//!    });
//!    set.insert(MyItem {
//!        id1: 1,
//!        id2: "ww".to_string(),
//!        ctx1: -2,
//!        ctx2: "cc".to_string(),
//!    });
//!    println!("{:?}", set);
//!    for v in set.iter() {
//!        println!("{:?}", v);
//!    }
//!    for v in set.iter_mut() {
//!        v.ctx1 = 0;
//!        println!("{:?}", v.id1);
//!        // In `iter_mut` IDs write will be prohibited
//!        // v.id1 = 0;
//!    }
//!    println!("{:?}", set);
//!    println!("{:?}", set.get(&MyItem::new_id(2, "www".to_string())));
//!    set.replace(MyItem {
//!        id1: 1,
//!        id2: "ww".to_string(),
//!        ctx1: -2,
//!        ctx2: "cc".to_string(),
//!    });
//!    println!("{:?}", set);
//!    for v in set.into_iter() {
//!        println!("{:?}", v);
//!    }
//!}
//!
//! ```
extern crate proc_macro;

mod expand;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// See more at [![github](https://img.shields.io/badge/github-main-blue?logo=github)](https://github.com/zao111222333/mut_set)
/// ```
///#[derive(Debug)]
///#[mut_set::derive::item]
///pub struct MyItem<T1, T2>
///where
///    T1: Sized,
///{
///    #[id]
///    pub(self) id1: usize,
///    pub(crate) ctx1: T1,
///    pub ctx2: T2,
///    #[id]
///    pub id2: String,
///}
///#[test]
///fn test() {
///    let mut set = mut_set::MutSet::new();
///    println!("{:?}", set);
///    set.insert(MyItem {
///        id1: 2,
///        id2: "www".to_string(),
///        ctx1: -1,
///        ctx2: "ccc".to_string(),
///    });
///    set.insert(MyItem {
///        id1: 1,
///        id2: "ww".to_string(),
///        ctx1: -2,
///        ctx2: "cc".to_string(),
///    });
///    println!("{:?}", set);
///    for v in set.iter() {
///        println!("{:?}", v);
///    }
///    for v in set.iter_mut() {
///        v.ctx1 = 0;
///        println!("{:?}", v.id1);
///        // In `iter_mut` IDs write will be prohibited
///        // v.id1 = 0;
///    }
///    println!("{:?}", set);
///    println!("{:?}", set.get(&MyItem::new_id(2, "www".to_string())));
///    set.replace(MyItem {
///        id1: 1,
///        id2: "ww".to_string(),
///        ctx1: -2,
///        ctx2: "cc".to_string(),
///    });
///    println!("{:?}", set);
///    for v in set.into_iter() {
///        println!("{:?}", v);
///    }
///}
/// ```
#[proc_macro_attribute]
pub fn item(args: TokenStream, tokens: TokenStream) -> TokenStream {
    let original = tokens.clone();
    // let args: DeriveInput = parse_macro_input!(args);
    let input = parse_macro_input!(tokens as DeriveInput);

    expand::readonly(args.into(), input)
        .unwrap_or_else(|e| {
            let original = proc_macro2::TokenStream::from(original);
            let compile_error = e.to_compile_error();
            quote! {
                #original
                #compile_error
            }
        })
        .into()
}
