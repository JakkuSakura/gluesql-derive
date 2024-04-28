use proc_macro::TokenStream;

use darling::{Error, FromDeriveInput};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DeriveInput;

use crate::field::{DeriveGluesqlRow, GluesqlField};

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
        let predicates = self.predicates_to()?;

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
    /// Generates any additional where clause predicates needed for the fields in this struct.
    pub fn predicates_to(&self) -> syn::Result<Vec<syn::__private::TokenStream2>> {
        let mut predicates = Vec::new();

        for field in self.fields() {
            field.add_predicates_to(&mut predicates)?;
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
    pub fn add_predicates_to(
        &self,
        predicates: &mut Vec<syn::__private::TokenStream2>,
    ) -> syn::Result<()> {
        let target_ty = &self.target_ty()?;

        predicates.push(if self.flatten {
            quote! (#target_ty: ::gluesql_derive::ToGlueSqlRow)
        } else {
            quote! (#target_ty: ::gluesql_derive::ToGlueSql)
        });

        Ok(())
    }

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
