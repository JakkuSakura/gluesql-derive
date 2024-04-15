use proc_macro::TokenStream;

use darling::{Error, FromDeriveInput};
use quote::quote;
use syn::DeriveInput;

use crate::field::DeriveGluesqlRow;

/// Fallible entry point for generating a `ToRow` implementation
pub fn try_derive_reflect_row(input: &DeriveInput) -> Result<TokenStream, Error> {
    let from_row_derive = DeriveGluesqlRow::from_derive_input(input)?;
    Ok(from_row_derive.generate_reflect_row()?)
}

impl DeriveGluesqlRow {
    /// Generate the `FromRow` implementation.
    fn generate_reflect_row(mut self) -> syn::Result<TokenStream> {
        self.validate()?;

        let ident = &self.ident;

        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let original_predicates = where_clause.clone().map(|w| &w.predicates).into_iter();
        let predicates = self.predicates()?;

        let fields = self
            .fields()
            .iter()
            .map(|f| f.ident.as_ref().unwrap().to_string())
            .collect::<Vec<_>>();

        Ok(quote! {
            impl #impl_generics ::gluesql_derive::ReflectGlueSqlRow for #ident #ty_generics where #(#original_predicates),* #(#predicates),* {
                fn columns() -> Vec<&'static str> {
                    vec![
                        #(#fields), *
                    ]
                }
            }
        }
        .into())
    }
}
