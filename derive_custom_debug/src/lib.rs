use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{Attribute, Expr, ExprLit, Fields, Lit, Meta, MetaNameValue};
use syn::{Data, DeriveInput, FieldsNamed, parse_macro_input, parse_quote, spanned::Spanned};

#[proc_macro_derive(CustomDebug, attributes(debug))]
pub fn derive_custom_debug(input: TokenStream) -> TokenStream {
    // Derive macros don't _replace_ the input item, they add to it.
    // If you return an empty `TokenStream`, the item is still created.
    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident;
    let ident_string = ident.to_string();

    let data = input.data;

    let Data::Struct(data) = data else {
        unimplemented!()
    };
    let Fields::Named(fields) = data.fields else {
        unimplemented!()
    };
    let fields_iter = fields.named.into_iter().map(|field| {
        let name = field.ident.as_ref().unwrap();
        let name_str = name.to_string();
        let debug_attr = field.attrs.into_iter().find(|attr| {
            let Ok(name_value) = attr.meta.require_name_value() else {
                return false;
            };
            name_value.path.is_ident("debug")
        });

        // If we've got a #[debug = <FORMAT_STR>], use it.
        match debug_attr {
            Some(Attribute {
                meta:
                    Meta::NameValue(MetaNameValue {
                        value:
                            Expr::Lit(ExprLit {
                                lit: Lit::Str(format_str),
                                ..
                            }),
                        ..
                    }),
                ..
            }) => quote!(.field(#name_str, &format!(#format_str, &self.#name))),
            _ => quote!(.field(#name_str, &self.#name)),
        }
    });

    quote!(
        impl Debug for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} {{", #name_str)
            }
        }
    )
    .into()

    // let fields_iter = match data {
    //     Data::Struct(data) => match data.fields {
    //         syn::Fields::Named(fields) => {
    //             let fields_iter = fields.named.iter().map(|f| {
    //                 let name = f.ident.as_ref().unwrap();
    //                 let name_str = name.to_string();
    //                 quote_spanned! { f.span() =>
    //                     .field(#name_str, &self.#name)
    //                 }
    //             });

    //             quote! {
    //                 impl std::fmt::Debug for #ident {
    //                     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //                         f.debug_struct(#ident_string)#(#fields_iter)*.finish()
    //                     }
    //                 }
    //             }
    //             .into()
    //         }
    //         syn::Fields::Unnamed(fields) => {
    //             let fields_iter = fields.unnamed.iter().enumerate().map(|(idx, f)| {
    //                 quote_spanned! { f.span() =>
    //                     .field(&self.#idx)
    //                 }
    //             });
    //             quote! {
    //                 impl std::fmt::Debug for #ident {
    //                     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //                         f.debug_tuple(#ident_string)#(#fields_iter)*.finish()
    //                     }
    //                 }
    //             }
    //             .into()
    //         }
    //         syn::Fields::Unit => quote! {
    //             impl std::fmt::Debug for #ident {
    //                 fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //                     f.debug_struct(#ident_string).finish()
    //                 }
    //             }
    //         }
    //         .into(),
    //     },
    //     Data::Enum(_) | Data::Union(_) => unimplemented!(),
    // }
}
