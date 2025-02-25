use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{Data, DeriveInput, FieldsNamed, parse_macro_input, parse_quote, spanned::Spanned};

#[proc_macro_derive(CustomDebug)]
pub fn derive_custom_debug(input: TokenStream) -> TokenStream {
    // Derive macros don't _replace_ the input item, they add to it.
    // If you return an empty `TokenStream`, the item is still created.
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let ident_string = ident.to_string();

    let data = input.data;

    match data {
        Data::Struct(data) => match data.fields {
            syn::Fields::Named(fields) => {
                let field_iter = fields.named.iter().map(|f| {
                    let name = f.ident.as_ref().unwrap();
                    let name_str = name.to_string();
                    quote_spanned! { f.span() =>
                        .field(#name_str, &self.#name)
                    }
                });

                quote! {
                    impl std::fmt::Debug for #ident {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            f.debug_struct(#ident_string)#(#field_iter)*.finish()
                        }
                    }
                }
                .into()
            }
            syn::Fields::Unnamed(fields_unnamed) => todo!(),
            syn::Fields::Unit => todo!(),
        },
        Data::Enum(_) | Data::Union(_) => unimplemented!(),
    }
}
