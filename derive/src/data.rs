#![allow(dead_code)]


// Adapted from Druid data.rs

// Copyright 2019 The Druid Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Data, DataEnum, DataStruct};

pub(crate) fn derive_data_impl(
    input: syn::DeriveInput,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    match &input.data {
        Data::Struct(s) => derive_struct(&input, s),
        Data::Enum(e) => derive_enum(&input, e),
        Data::Union(u) => Err(syn::Error::new(
            u.union_token.span(),
            "Data implementations cannot be derived from unions",
        )),
    }
}

fn derive_struct(
    input: &syn::DeriveInput,
    _s: &DataStruct,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let ident = &input.ident;
    let impl_generics = generics_bounds(&input.generics);
    let (_, ty_generics, where_clause) = &input.generics.split_for_impl();

    let res = quote! {
        impl<#impl_generics> tuix::Node for #ident #ty_generics #where_clause {

        }
    };

    Ok(res)
}

fn derive_enum(
    input: &syn::DeriveInput,
    _s: &DataEnum,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let ident = &input.ident;
    let impl_generics = generics_bounds(&input.generics);
    let (_, ty_generics, where_clause) = &input.generics.split_for_impl();

    let res = quote! {
        impl<#impl_generics> tuix::Node for #ident #ty_generics #where_clause {

        }
    };

    Ok(res)
}


fn generics_bounds(generics: &syn::Generics) -> proc_macro2::TokenStream {
    let res = generics.params.iter().map(|gp| {
        use syn::GenericParam::*;
        match gp {
            Type(ty) => {
                let ident = &ty.ident;
                let bounds = &ty.bounds;
                if bounds.is_empty() {
                    quote_spanned!(ty.span()=> #ident : tuix::Node)
                } else {
                    quote_spanned!(ty.span()=> #ident : #bounds + tuix::Node)
                }
            }
            Lifetime(lf) => quote!(#lf),
            Const(cst) => quote!(#cst),
        }
    });

    quote!( #( #res, )* )
}