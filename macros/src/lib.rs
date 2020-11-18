use heck::SnakeCase;
use proc_macro2::Ident;
use quote::quote;
use syn::export::TokenStream;
use syn::{parse_macro_input, Data, DeriveInput, Type};

#[proc_macro_derive(Sql)]
pub fn sql(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let data = if let Data::Struct(s) = &input.data {
        s
    } else {
        panic!("Expected a struct");
    };

    let ident = &input.ident;
    let generics = &input.generics;

    let table_name = input.ident.to_string().to_snake_case();

    let columns: Vec<(Ident, &Type)> = data
        .fields
        .iter()
        .map(|f| (f.ident.clone().unwrap(), &f.ty))
        .collect();

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let expanded = quote! {
        impl #impl_generics crate::db::DbObject for #ident #ty_generics #where_clause {
            fn create_table(db: &rusqlite::Connection) -> Result<(), crate::Error> {
                unimplemented!();
            }

            fn save(&self, db: &rusqlite::Connection) -> Result<(), crate::Error> {
                unimplemented!();
            }

            fn reload(&mut self, db: &rusqlite::Connection) -> Result<&mut Self, crate::Error> {
                unimplemented!();
            }

            fn destroy(&self, db: &rusqlite::Connection) -> Result<(), crate::Error> {
                unimplemented!();
            }
        }
    };

    expanded.into()
}
