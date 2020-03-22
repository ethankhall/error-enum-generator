extern crate proc_macro;

use proc_macro_error::abort;

use proc_macro::TokenStream;
use quote::quote;

use strum_helpers::{extract_meta, MetaIteratorHelpers};

pub(crate) fn impl_generate_error_code(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let type_meta = extract_meta(&ast.attrs);

    let variants = match ast.data {
        syn::Data::Enum(ref v) => &v.variants,
        _ => panic!("PrettyError only works on Enums"),
    };

    let mut error_codes = Vec::new();
    let mut description = Vec::new();
    let mut display = Vec::new();

    let prefix = match type_meta.find_unique_property("error_enum", "prefix") {
        Some(prefix) => prefix,
        None => {
            abort!(ast.ident, "'prefix' is required on error_enum");
        }
    };

    for (idx, variant) in variants.iter().enumerate() {
        use syn::Fields::*;
        let ident = &variant.ident;
        let variant_meta = extract_meta(&variant.attrs);

        let error_description = match variant_meta.find_unique_property("error_enum", "description")
        {
            Some(desc) => desc,
            None => {
                abort!(
                    variant.ident,
                    "'description' is required on all fields in PrettyError"
                );
            }
        };

        let params = match variant.fields {
            Unit => quote! {},
            Unnamed(..) => quote! { (..) },
            Named(..) => quote! { {..} },
        };

        let code = format!("{}-{:03}", prefix, idx + 1);
        let error_description = format!("{}.", error_description);

        error_codes.push(quote! { #name::#ident #params => #code});
        description.push(quote! { #name::#ident #params => #error_description});

        let (display_params, display_code) = match variant.fields {
            Unit => (
                quote! {},
                quote! { write!(f, "({}): {}", error_code, desc) },
            ),
            Unnamed(ref fields) => {
                if fields.unnamed.len() != 1 {
                    abort!(variant.ident, "Only one varable is supported");
                }

                (
                    quote! { (err) },
                    quote! { write!(f, "({}): {} Detailed Error: {:?}", error_code, desc, err) },
                )
            }
            Named(ref fields) => {
                if fields.named.len() != 1 {
                    abort!(variant.ident, "Only one varable is supported");
                }

                let field = fields
                    .named
                    .first()
                    .map(|field| field.ident.as_ref().unwrap())
                    .unwrap();

                (
                    quote! { { #field } },
                    quote! { write!(f, "({}): {} Detailed Error: {:?}", error_code, desc, #field) },
                )
            }
        };

        display.push(quote! { #name::#ident #display_params => #display_code });
    }

    let tokens = quote! {
        impl ::error_enum::PrettyError for #name {
            fn get_error_code(&self) -> &str {
                match self {
                    #(#error_codes),*
                }
            }

            fn description(&self) -> &str {
                match self {
                    #(#description),*
                }
            }
        }


        impl ::std::fmt::Display for #name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let error_code = self.get_error_code();
                let desc = self.description();
                match self {
                    #(#display),*
                }
            }
        }
    };

    tokens.into()
}

pub(crate) fn impl_generate_error_wrapper(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let variants = match ast.data {
        syn::Data::Enum(ref v) => &v.variants,
        _ => panic!("PrettyError only works on Enums"),
    };

    let mut deref = Vec::new();
    let mut display = Vec::new();
    let mut from_impl = Vec::new();

    for variant in variants.iter() {
        use syn::Fields::*;
        let ident = &variant.ident;

        match variant.fields {
            Unit => {
                abort!(variant.ident, "One and only one params required");
            }
            Unnamed(ref fields) => {
                if fields.unnamed.len() != 1 {
                    abort!(variant.ident, "One and only one params required");
                }

                // Generate a from impl From
                match &fields.unnamed.first().unwrap().ty {
                    syn::Type::Path(type_path) => {
                        if type_path.path.segments.len() == 1 {
                            let arg_name = &type_path.path.segments.first().unwrap().ident;
                            from_impl.push(quote! {
                                impl std::convert::From<#arg_name> for #name {
                                    fn from(error: #arg_name) -> Self {
                                        #name::#ident(error)
                                    }
                                }
                            });
                        }
                    }
                    _ => {}
                }

                display.push(quote! { #name::#ident (param) => write!(f, "{}", param) });
                deref.push(quote! { #name::#ident (param) => param });
            }
            Named(ref fields) => {
                if fields.named.len() != 1 {
                    abort!(variant.ident, "One and only one params required");
                }

                let field = fields
                    .named
                    .first()
                    .map(|field| field.ident.as_ref().unwrap())
                    .unwrap();

                display.push(quote! { #name::#ident { #field } => write!(f, "{}", #field) });
                deref.push(quote! { #name::#ident (#field) => #field });
            }
        }
    }

    let tokens = quote! {
        impl ::std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display),*
                }
            }
        }

        #(#from_impl)*

        impl ::std::ops::Deref for #name {
            type Target = dyn ::error_enum::PrettyError;

            fn deref(&self) -> &Self::Target {
                match self {
                    #(#deref),*
                }
            }
        }
    };

    tokens.into()
}
