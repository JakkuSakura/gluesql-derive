use proc_macro2::TokenStream;

use darling::{Error, FromDeriveInput};
use quote::quote;
use syn::DeriveInput;

use crate::field::{DeriveGluesqlRow, GluesqlField};

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
        let predicates = self.predicates_reflect()?;

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
    /// Generates any additional where clause predicates needed for the fields in this struct.
    pub fn predicates_reflect(&self) -> syn::Result<Vec<syn::__private::TokenStream2>> {
        let mut predicates = Vec::new();

        for field in self.fields() {
            field.add_predicates_reflect(&mut predicates)?;
        }

        Ok(predicates)
    }
}
impl GluesqlField {
    /// Pushes the needed where clause predicates for this field.
    ///
    /// By default this is `T: postgres::types::FromSql`,
    /// when using `flatten` it's: `T: postgres_from_row::FromRow`
    /// and when using either `from` or `try_from` attributes it additionally pushes this bound:
    /// `T: std::convert::From<R>`, where `T` is the type specified in the struct and `R` is the
    /// type specified in the `[try]_from` attribute.
    pub fn add_predicates_reflect(
        &self,
        predicates: &mut Vec<syn::__private::TokenStream2>,
    ) -> syn::Result<()> {
        let target_ty = &self.target_ty()?;

        predicates.push(if self.flatten {
            quote! (#target_ty: ::gluesql_derive::ReflectGlueSqlRow)
        } else {
            quote! (#target_ty: ::gluesql_derive::ReflectGlueSql)
        });

        Ok(())
    }
}
