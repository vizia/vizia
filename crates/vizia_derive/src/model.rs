use proc_macro2::Ident;
use quote::quote;
use syn::spanned::Spanned;
use syn::Data;

pub(crate) fn derive_model_impl(
    input: syn::DeriveInput,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    match &input.data {
        Data::Struct(_) => derive_struct(&input),
        Data::Enum(e) => Err(syn::Error::new(
            e.enum_token.span(),
            "Model implementations cannot be derived from enums",
        )),
        Data::Union(u) => Err(syn::Error::new(
            u.union_token.span(),
            "Model implementations cannot be derived from unions",
        )),
    }
}

fn derive_struct(input: &syn::DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    let struct_type = &input.ident;
    let ray_type = Ident::new(&(struct_type.to_string() + "Setter"), input.ident.span());

    Ok(quote! {
        impl Model for #struct_type {
            fn event(&mut self, _: &mut EventContext, event: &mut Event) {
                event.take::<#ray_type>().map(|setter| setter.apply(self));
            }
        }
    })
}
