use proc_macro::TokenStream;

use darling::{Error, FromDeriveInput};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DeriveInput;

use crate::field::{DeriveGluesqlRow, FromRowField};

/// Fallible entry point for generating a `ToRow` implementation
pub fn try_derive_to_row(input: &DeriveInput) -> Result<TokenStream, Error> {
    let from_row_derive = DeriveGluesqlRow::from_derive_input(input)?;
    Ok(from_row_derive.generate_to_row()?)
}

impl DeriveGluesqlRow {
    /// Generate the `ToRow` implementation.
    fn generate_to_row(mut self) -> syn::Result<TokenStream> {
        self.validate()?;

        let ident = &self.ident;

        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let original_predicates = where_clause.clone().map(|w| &w.predicates).into_iter();
        let predicates = self.predicates()?;

        let to_row_fields = self
            .fields()
            .iter()
            .map(|f| f.generate_to_row())
            .collect::<syn::Result<Vec<_>>>()?;

        Ok(quote! {
            impl #impl_generics ::gluesql_derive::ToGlueSqlRow for #ident #ty_generics where #(#original_predicates),* #(#predicates),* {
                fn to_gluesql_row(&self) -> Vec<::gluesql_derive::gluesql_core::ast_builder::ExprNode<'static>> {
                    vec![
                        #(#to_row_fields), *
                    ]
                }
            }
        }
        .into())
    }
}

impl FromRowField {
    /// Generate the line needed to retrieve this field from a row when calling `from_row`.
    fn generate_to_row(&self) -> syn::Result<TokenStream2> {
        let ident = self.ident.as_ref().unwrap();
        let field_ty = &self.ty;

        let mut base = if self.flatten {
            unimplemented!("flatten field not supported")
        } else {
            quote!(&self.#ident)
        };

        base = quote!(<#field_ty as ::gluesql_derive::ToGlueSql>::to_gluesql(#base));

        Ok(quote!(#base))
    }
}
