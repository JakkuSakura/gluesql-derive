use darling::ast::Data;
use darling::{Error, FromDeriveInput, FromField};
use quote::{quote, ToTokens};
use syn::__private::TokenStream2;

/// Main struct for deriving `FromRow` for a struct.
#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(gluesql),
    forward_attrs(allow, doc, cfg),
    supports(struct_named)
)]
pub struct DeriveGluesqlRow {
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub data: Data<(), FromRowField>,
}

impl DeriveGluesqlRow {
    /// Validates all fields
    pub fn validate(&mut self) -> syn::Result<()> {
        match &mut self.data {
            Data::Struct(fields) => {
                fields.fields.iter_mut().enumerate().for_each(|(i, x)| {
                    x.index = i;
                });
            }
            _ => panic!("invalid shape"),
        }
        for field in self.fields() {
            field.validate()?;
        }

        Ok(())
    }

    /// Generates any additional where clause predicates needed for the fields in this struct.
    pub fn predicates(&self) -> syn::Result<Vec<TokenStream2>> {
        let mut predicates = Vec::new();

        for field in self.fields() {
            field.add_predicates(&mut predicates)?;
        }

        Ok(predicates)
    }

    /// Provides a slice of this struct's fields.
    pub fn fields(&self) -> &[FromRowField] {
        match &self.data {
            Data::Struct(fields) => &fields.fields,
            _ => panic!("invalid shape"),
        }
    }
}
/// A single field inside a struct that derives `FromRow`
#[derive(Debug, FromField)]
#[darling(attributes(from_row), forward_attrs(allow, doc, cfg))]
pub struct FromRowField {
    #[darling(default)]
    pub index: usize,
    /// The identifier of this field.
    pub ident: Option<syn::Ident>,
    /// The type specified in this field.
    pub ty: syn::Type,
    /// Wether to flatten this field. Flattening means calling the `FromRow` implementation
    /// of `self.ty` instead of extracting it directly from the row.
    #[darling(default)]
    pub flatten: bool,
    /// Optionaly use this type as the target for `FromRow` or `FromSql`, and then
    /// call `TryFrom::try_from` to convert it the `self.ty`.
    pub try_from: Option<String>,
    /// Optionaly use this type as the target for `FromRow` or `FromSql`, and then
    /// call `From::from` to convert it the `self.ty`.
    pub from: Option<String>,
    /// Override the name of the actual sql column instead of using `self.ident`.
    /// Is not compatible with `flatten` since no column is needed there.
    pub rename: Option<String>,
}

impl FromRowField {
    /// Checks wether this field has a valid combination of attributes
    pub fn validate(&self) -> syn::Result<()> {
        if self.from.is_some() && self.try_from.is_some() {
            return Err(Error::custom(
                r#"can't combine `#[from_row(from = "..")]` with `#[from_row(try_from = "..")]`"#,
            )
            .into());
        }

        if self.rename.is_some() && self.flatten {
            return Err(Error::custom(
                r#"can't combine `#[from_row(flatten)]` with `#[from_row(rename = "..")]`"#,
            )
            .into());
        }

        Ok(())
    }

    /// Returns a tokenstream of the type that should be returned from either
    /// `FromRow` (when using `flatten`) or `FromSql`.
    pub fn target_ty(&self) -> syn::Result<TokenStream2> {
        if let Some(from) = &self.from {
            Ok(from.parse()?)
        } else if let Some(try_from) = &self.try_from {
            Ok(try_from.parse()?)
        } else {
            Ok(self.ty.to_token_stream())
        }
    }

    /// Returns the name that maps to the actuall sql column
    /// By default this is the same as the rust field name but can be overwritten by `#[from_row(rename = "..")]`.
    pub fn column_name(&self) -> String {
        self.rename
            .as_ref()
            .map(Clone::clone)
            .unwrap_or_else(|| self.ident.as_ref().unwrap().to_string())
    }

    /// Pushes the needed where clause predicates for this field.
    ///
    /// By default this is `T: postgres::types::FromSql`,
    /// when using `flatten` it's: `T: postgres_from_row::FromRow`
    /// and when using either `from` or `try_from` attributes it additionally pushes this bound:
    /// `T: std::convert::From<R>`, where `T` is the type specified in the struct and `R` is the
    /// type specified in the `[try]_from` attribute.
    pub fn add_predicates(&self, predicates: &mut Vec<TokenStream2>) -> syn::Result<()> {
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
