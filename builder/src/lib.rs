use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DataStruct, DeriveInput, Ident, Type};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let mut definition = parse_macro_input!(input as DeriveInput);
    let orig_ident = definition.ident.clone();
    let builder_ident = Ident::new(&format!("{}Builder", &orig_ident), orig_ident.span());
    definition.ident = builder_ident.clone();

    let mut idents: Vec<&Ident> = vec![];
    let mut types: Vec<Type> = vec![];
    match &mut definition.data {
        syn::Data::Struct(DataStruct { fields, .. }) => {
            for field in fields {
                let ty = &field.ty;
                let new_ty: Type = parse_quote!(Option<#ty>);
                types.push(std::mem::replace(&mut field.ty, new_ty));
                idents.extend(&field.ident);
            }
        }
        syn::Data::Enum(..) => {
            todo!("Enum builders should be possible...?")
        }
        syn::Data::Union(..) => {
            todo!("If enum builders are possible, union builders should be possible, too.")
        }
    }

    quote! {
        impl #orig_ident {
            fn builder() -> #builder_ident {
                #builder_ident {
                    #(#idents: None),*
                }
            }
        }
        impl #builder_ident {
            #(pub fn #idents(&mut self, #idents: #types) {
                self.#idents = Some(#idents);
            })*
            fn build(self) -> Result<#orig_ident, String> {
                #(if self.#idents.is_none() {
                    const E: &str = stringify!(#builder_ident missing field #idents);
                    Err(String::from(E))?;
                })*
                Ok(#orig_ident { #(#idents: self.#idents.unwrap(),)* })
            }
        }
        #definition
    }
    .into()
}
