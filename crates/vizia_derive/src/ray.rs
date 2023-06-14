use crate::attr::{FieldKind, Fields, RayAttrs};
use crate::lens::{char_has_case, increase_visibility};
use proc_macro2::Ident;
use quote::quote;
use syn::spanned::Spanned;
use syn::Data;

pub(crate) fn derive_ray_impl(
    input: syn::DeriveInput,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    match &input.data {
        Data::Struct(_) => derive_struct(&input),
        Data::Enum(e) => Err(syn::Error::new(
            e.enum_token.span(),
            "Setter implementations cannot be derived from enums",
        )),
        Data::Union(u) => Err(syn::Error::new(
            u.union_token.span(),
            "Setter implementations cannot be derived from unions",
        )),
    }
}

fn derive_struct(input: &syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    let enum_type = &input.ident;
    let ray_name = Ident::new(&(enum_type.to_string() + "Setter"), enum_type.span());

    let module_vis = &input.vis;
    let enum_vis = increase_visibility(module_vis);

    let fields = if let syn::Data::Struct(syn::DataStruct { fields, .. }) = &input.data {
        Fields::<RayAttrs>::parse_ast(fields)?
    } else {
        return Err(syn::Error::new(
            input.span(),
            "Setter implementations can only be derived from structs with named fields",
        ));
    };

    if fields.kind != FieldKind::Named {
        return Err(syn::Error::new(
            input.span(),
            "Setter implementations can only be derived from structs with named fields",
        ));
    }

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut variants = vec![];
    let mut swap_arms = vec![];
    let mut apply_arms = vec![];

    // Define ray types for each field, define ray match arms for each field
    for f in fields.iter().filter(|f| !f.attrs.ignore) {
        let field_name = &f.ident.unwrap_named();
        let field_ty = &f.ty;
        let variant_name = Ident::new(&to_camel_case(&field_name.to_string()), field_name.span());

        variants.push(quote! { #variant_name(#field_ty) });
        swap_arms.push(
            quote! { #ray_name::#variant_name(v) => std::mem::swap(v, &mut source.#field_name) },
        );
        apply_arms.push(quote! { #ray_name::#variant_name(v) => source.#field_name = v });
    }

    // let enum_docs = format!("Setter enum for [`{ty}`](super::{ty}).", ty = enum_type,);

    let expanded = quote! {
        // #[doc = #enum_docs]
        #[allow(non_camel_case_types)]
        #[derive(Debug)]
        #enum_vis enum #ray_name #ty_generics #where_clause {
            #(#variants),*
        }

        impl #impl_generics Setter for #ray_name #ty_generics #where_clause {
            type Source = #enum_type #ty_generics;

            fn swap(&mut self, source: &mut Self::Source) {
                match self {
                    #(#swap_arms),*
                }
            }

            fn apply(self, source: &mut Self::Source) {
                match self {
                    #(#apply_arms),*
                }
            }
        }
    };

    Ok(expanded)
}

// once again, stolen from rustc
fn to_camel_case(s: &str) -> String {
    s.trim_matches('_')
        .split('_')
        .filter(|component| !component.is_empty())
        .map(|component| {
            let mut camel_cased_component = String::new();

            let mut new_word = true;
            let mut prev_is_lower_case = true;

            for c in component.chars() {
                // Preserve the case if an uppercase letter follows a lowercase letter, so that
                // `camelCase` is converted to `CamelCase`.
                if prev_is_lower_case && c.is_uppercase() {
                    new_word = true;
                }

                if new_word {
                    camel_cased_component.extend(c.to_uppercase());
                } else {
                    camel_cased_component.extend(c.to_lowercase());
                }

                prev_is_lower_case = c.is_lowercase();
                new_word = false;
            }

            camel_cased_component
        })
        .fold((String::new(), None), |(acc, prev): (String, Option<String>), next| {
            // separate two components with an underscore if their boundary cannot
            // be distinguished using an uppercase/lowercase case distinction
            let join = if let Some(prev) = prev {
                let l = prev.chars().last().unwrap();
                let f = next.chars().next().unwrap();
                !char_has_case(l) && !char_has_case(f)
            } else {
                false
            };
            (acc + if join { "_" } else { "" } + &next, Some(next))
        })
        .0
}

fn is_snake_case(ident: &str) -> bool {
    if ident.is_empty() {
        return true;
    }
    let ident = ident.trim_start_matches('\'');
    let ident = ident.trim_matches('_');

    let mut allow_underscore = true;
    ident.chars().all(|c| {
        allow_underscore = match c {
            '_' if !allow_underscore => return false,
            '_' => false,
            // It would be more obvious to use `c.is_lowercase()`,
            // but some characters do not have a lowercase form
            c if !c.is_uppercase() => true,
            _ => return false,
        };
        true
    })
}
