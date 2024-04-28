use proc_macro::TokenStream;

use darling::{Error, FromDeriveInput};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::DeriveInput;

use crate::field::{DeriveGluesqlRow, GluesqlField};

/// Fallible entry point for generating a `FromRow` implementation
pub fn try_derive_from_row(input: &DeriveInput) -> Result<TokenStream, Error> {
    let from_row_derive = DeriveGluesqlRow::from_derive_input(input)?;
    Ok(from_row_derive.generate_from_row()?)
}

impl DeriveGluesqlRow {
    /// Generate the `FromRow` implementation.
    fn generate_from_row(mut self) -> syn::Result<TokenStream> {
        self.validate()?;

        let ident = &self.ident;

        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let original_predicates = where_clause.clone().map(|w| &w.predicates).into_iter();
        let predicates = self.predicates_from()?;

        let from_row_fields = self
            .fields()
            .iter()
            .map(|f| f.generate_from_row())
            .collect::<syn::Result<Vec<_>>>()?;

        Ok(quote! {
            impl #impl_generics ::gluesql_derive::FromGlueSqlRow for #ident #ty_generics where #(#original_predicates),* #(#predicates),* {
                fn from_gluesql_row(labels: &[String], row: Vec<::gluesql_derive::gluesql_core::prelude::Value>) -> Result<Self, ::gluesql_derive::Error> {
                    let mut row = row.into_iter();
                    let this = Self {
                        #(#from_row_fields), *
                    };
                    drop(row);
                    Ok(this)
                }

            }
        }
        .into())
    }

    /// Generates any additional where clause predicates needed for the fields in this struct.
    pub fn predicates_from(&self) -> syn::Result<Vec<syn::__private::TokenStream2>> {
        let mut predicates = Vec::new();

        for field in self.fields() {
            field.add_predicates_from(&mut predicates)?;
        }

        Ok(predicates)
    }
}

impl GluesqlField {
    /// Generate the line needed to retrievee this field from a row when calling `from_row`.
    fn generate_from_row(&self) -> syn::Result<TokenStream2> {
        let ident = self.ident.as_ref().unwrap();
        let column_name = self.column_name();
        let index = self.index;
        let field_ty = &self.ty;
        let target_ty = self.target_ty()?;

        let mut base = if self.flatten {
            unimplemented!("flatten field not supported")
        } else {
            quote!({
                if labels.get(#index).map(|x| x.as_str()) != Some(#column_name) {
                    return Err(::gluesql_derive::Error::InvalidFieldName(#index, #column_name, labels.get(#index).cloned().unwrap_or_default()));
                }
                row.next().ok_or(::gluesql_derive::Error::InvalidExtract(#index, #column_name))?
            })
        };

        if self.from.is_some() {
            base = quote!(<#field_ty as std::convert::From<#target_ty>>::from(#base));
        } else if self.try_from.is_some() {
            base = quote!(<#field_ty as std::convert::TryFrom<#target_ty>>::try_from(#base).expect("could not convert column"));
        } else {
            base = quote!(<#field_ty as ::gluesql_derive::FromGlueSql>::from_gluesql(#base)?);
        }

        Ok(quote!(#ident: #base))
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
    pub fn add_predicates_from(
        &self,
        predicates: &mut Vec<syn::__private::TokenStream2>,
    ) -> syn::Result<()> {
        let target_ty = &self.target_ty()?;
        let ty = &self.ty;

        predicates.push(if self.flatten {
            quote! (#target_ty: ::gluesql_derive::FromRow)
        } else {
            quote! (#target_ty: ::gluesql_derive::FromGlueSql)
        });

        if self.from.is_some() {
            predicates.push(quote!(#ty: std::convert::From<#target_ty>))
        } else if self.try_from.is_some() {
            let try_from = quote!(std::convert::TryFrom<#target_ty>);

            predicates.push(quote!(#ty: #try_from));
            predicates.push(
                quote!(::gluesql_derive::Error: std::convert::From<<#ty as #try_from>::Error>),
            );
            predicates.push(quote!(<#ty as #try_from>::Error: std::fmt::Debug));
        }

        Ok(())
    }
}
