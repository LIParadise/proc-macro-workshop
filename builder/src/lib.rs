use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DataStruct, DeriveInput, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let mut definition = parse_macro_input!(input as DeriveInput);
    let orig_ident = definition.ident.clone();
    let builder_ident = Ident::new(&format!("{}Builder", &orig_ident), orig_ident.span());
    definition.ident = builder_ident.clone();

    let mut instance_body: Vec<&Ident> = vec![];
    match &mut definition.data {
        syn::Data::Struct(DataStruct { fields, .. }) => {
            for field in fields {
                let ty = &field.ty;
                field.ty = parse_quote!(Option<#ty>);
                instance_body.extend(&field.ident);
            }
        }
        syn::Data::Enum(..) => {
            todo!("Enum builders should be possible...?")
        }
        syn::Data::Union(..) => {
            todo!("If enum builders are possible, union builders should be possible, too.")
        }
    }
    let instance_body = instance_body.iter();

    quote! {
        impl #orig_ident {
            fn builder() -> #builder_ident {
                #builder_ident {
                    #(#instance_body: None),*
                }
            }
        }
        #definition
    }
    .into()
}
