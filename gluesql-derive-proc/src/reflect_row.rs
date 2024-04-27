use proc_macro2::TokenStream;

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
    /// Generate the DDL for the struct.
    /// example:
    /// CREATE TABLE IF NOT EXISTS {} (
    ///    id UINT64 NOT NULL,
    ///    username TEXT NOT NULL
    /// );
    ///
    fn get_ddl(&self) -> TokenStream {
        let fields = self
            .fields()
            .iter()
            .map(|f| f.ident.as_ref().unwrap().to_string());
        let tys = self.fields().iter().map(|f| f.ty.clone());

        quote! {{
            let mut ddl = "".to_string();
            ddl.push_str("CREATE TABLE IF NOT EXISTS ");
            ddl.push_str(table);
            ddl.push_str(" (\n");
            #(
                ddl.push_str(#fields);
                ddl.push_str(" ");
                ddl.push_str(&<#tys as ::gluesql_derive::ReflectGlueSql>::reflect_gluesql_type_with_nullability());
            ) {
                ddl.push_str(", \n");
            } *
            ddl.push_str(");");
            ddl
        }}
    }
    fn get_columns(&self) -> TokenStream {
        let fields = self.fields();
        let columns = fields.iter().map(|f| f.ident.as_ref().unwrap().to_string());
        quote! {
            vec![#(#columns),*]
        }
    }
    /// Generate the `FromRow` implementation.
    fn generate_reflect_row(mut self) -> syn::Result<TokenStream> {
        self.validate()?;

        let ident = &self.ident;

        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let original_predicates = where_clause.clone().map(|w| &w.predicates).into_iter();
        let predicates = self.predicates()?;

        let ddl = self.get_ddl();
        let columns = self.get_columns();
        Ok(quote! {
            impl #impl_generics ::gluesql_derive::ReflectGlueSqlRow for #ident #ty_generics where #(#original_predicates),* #(#predicates),* {
                fn get_ddl(table: &str) -> String {
                    #ddl
                }
                fn columns() -> Vec<&'static str> {
                    #columns
                }
            }
        }
        .into())
    }
}
