use darling::ast::Data;
use darling::{Error, FromDeriveInput, FromField};
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;

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
    pub data: Data<(), GluesqlField>,
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

    /// Provides a slice of this struct's fields.
    pub fn fields(&self) -> &[GluesqlField] {
        match &self.data {
            Data::Struct(fields) => &fields.fields,
            _ => panic!("invalid shape"),
        }
    }
}
/// A single field inside a struct that derives `FromRow`
#[derive(Debug, FromField)]
#[darling(attributes(from_row), forward_attrs(allow, doc, cfg))]
pub struct GluesqlField {
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

impl GluesqlField {
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
}
