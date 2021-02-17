extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Data, DeriveInput, Fields};

#[proc_macro_derive(BlockData)]
pub fn block_data_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("Parsing struct for BlockData macro failed");

    impl_block_data(&ast)
}

fn impl_block_data(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let field;

    if let Data::Struct(data) = &ast.data {
        if let Fields::Named(named_fields) = &data.fields {
            if named_fields.named.len() != 1 {
                panic!("Can't derive BlockData for struct with more than 1 named field")
            };
            field = named_fields
                .named
                .first()
                .unwrap()
                .to_owned()
                .ident
                .unwrap();
        } else {
            panic!("Can't derive BlockData for a struct with no named fields")
        }
    } else {
        panic!("Can't derive BlockData for a non struct")
    }

    let gen = quote! {
        impl BlockData for #name {
            fn from_raw(raw: &RawBlockStateProperties) -> Option<Self>
            where
                Self: Sized,
            {
                Some(Self { #field: raw.#field? })
            }

            fn apply(&self, raw: &mut RawBlockStateProperties) {
                raw.#field.replace(self.#field);
            }
        }
    };

    gen.into()
}
