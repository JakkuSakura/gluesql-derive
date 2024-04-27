use proc_macro::TokenStream;

use crate::from_row::try_derive_from_row;
use crate::to_row::try_derive_to_row;
use syn::{parse_macro_input, DeriveInput};

mod field;
mod from_row;
mod reflect_row;
mod to_row;

#[proc_macro_derive(ReflectGlueSqlRow, attributes(gluesql))]
pub fn derive_reflect_row(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    reflect_row::try_derive_reflect_row(&derive_input)
        .unwrap_or_else(|err| err.write_errors().into())
        .into()
}

#[proc_macro_derive(FromGlueSqlRow, attributes(gluesql))]
pub fn derive_from_row(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    try_derive_from_row(&derive_input).unwrap_or_else(|err| err.write_errors().into())
}

#[proc_macro_derive(ToGlueSqlRow, attributes(gluesql))]
pub fn derive_to_from_row(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    try_derive_to_row(&derive_input).unwrap_or_else(|err| err.write_errors().into())
}
