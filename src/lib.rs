#![doc = include_str!("../README.md")]

use std::fmt::Debug;

#[doc(hidden)]
pub use gluesql_core;
use gluesql_core::ast_builder::ExprNode;
use gluesql_core::prelude::*;

pub use gluesql_derive_proc::{FromGlueSqlRow, ReflectGlueSqlRow, ToGlueSqlRow};

mod from_gluesql;
pub use from_gluesql::FromGlueSql;
mod to_gluesql;
pub use to_gluesql::ToGlueSql;
mod reflect_gluesql;
pub use reflect_gluesql::ReflectGlueSql;
#[derive(thiserror::Error)]
pub enum Error {
    #[error("could not convert into type {0}: {1:?}")]
    InvalidConversion(&'static str, Value),
    #[error("could not extract field: {0} {1:?}")]
    InvalidExtract(usize, &'static str),
    #[error("expected field {0} {1:?}, but actual label is {2:?}")]
    InvalidFieldName(usize, &'static str, String),
}
impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
pub trait ReflectGlueSqlRow {
    fn get_ddl(table: &str) -> String;
    fn columns() -> Vec<&'static str>;
}
pub trait FromGlueSqlRow: Sized {
    fn from_gluesql_row(labels: &[String], row: Vec<Value>) -> Result<Self, Error>;
    fn from_gluesql_rows(labels: &[String], rows: Vec<Vec<Value>>) -> Result<Vec<Self>, Error> {
        rows.into_iter()
            .map(|row| Self::from_gluesql_row(labels, row))
            .collect()
    }
}

pub trait ToGlueSqlRow {
    /// statically define expression
    fn to_gluesql_row(&self) -> Vec<ExprNode<'static>>;
}
