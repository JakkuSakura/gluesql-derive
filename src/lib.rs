#[doc(hidden)]
pub use gluesql_core;
use gluesql_core::prelude::*;
pub use gluesql_derive_proc::FromGlueSqlRow;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("could not convert into type {0}: {1}")]
    InvalidConversion(&'static str, String),
    #[error("could not extract field: {0} {1}")]
    InvalidExtract(usize, &'static str),
}

pub trait FromGlueSqlRow: Sized {
    fn from_gluesql_row(labels: &[String], row: Vec<Value>) -> Result<Self, Error>;
    fn from_gluesql_rows(labels: &[String], rows: Vec<Vec<Value>>) -> Result<Vec<Self>, Error> {
        rows.into_iter()
            .map(|row| Self::from_gluesql_row(labels, row))
            .collect()
    }
}
pub trait FromGlueSql: Sized {
    fn from_gluesql(value: &Value) -> Result<Self, Error>;
}
impl FromGlueSql for i8 {
    fn from_gluesql(value: &Value) -> Result<Self, Error> {
        match value {
            Value::I8(i) => Ok(*i),
            _ => Err(Error::InvalidConversion("i8", format!("{:?}", value))),
        }
    }
}
impl FromGlueSql for i16 {
    fn from_gluesql(value: &Value) -> Result<Self, Error> {
        match value {
            Value::I16(i) => Ok(*i),
            _ => Err(Error::InvalidConversion("i16", format!("{:?}", value))),
        }
    }
}
impl FromGlueSql for i32 {
    fn from_gluesql(value: &Value) -> Result<Self, Error> {
        match value {
            Value::I32(i) => Ok(*i),
            _ => Err(Error::InvalidConversion("i32", format!("{:?}", value))),
        }
    }
}
impl FromGlueSql for i64 {
    fn from_gluesql(value: &Value) -> Result<Self, Error> {
        match value {
            Value::I64(i) => Ok(*i),
            _ => Err(Error::InvalidConversion("i64", format!("{:?}", value))),
        }
    }
}
impl FromGlueSql for u8 {
    fn from_gluesql(value: &Value) -> Result<Self, Error> {
        match value {
            Value::U8(i) => Ok(*i),
            _ => Err(Error::InvalidConversion("u8", format!("{:?}", value))),
        }
    }
}
impl FromGlueSql for u16 {
    fn from_gluesql(value: &Value) -> Result<Self, Error> {
        match value {
            Value::U16(i) => Ok(*i),
            _ => Err(Error::InvalidConversion("u16", format!("{:?}", value))),
        }
    }
}
impl FromGlueSql for u32 {
    fn from_gluesql(value: &Value) -> Result<Self, Error> {
        match value {
            Value::U32(i) => Ok(*i),
            _ => Err(Error::InvalidConversion("u32", format!("{:?}", value))),
        }
    }
}
impl FromGlueSql for u64 {
    fn from_gluesql(value: &Value) -> Result<Self, Error> {
        match value {
            Value::U64(i) => Ok(*i),
            _ => Err(Error::InvalidConversion("u64", format!("{:?}", value))),
        }
    }
}
impl FromGlueSql for f32 {
    fn from_gluesql(value: &Value) -> Result<Self, Error> {
        match value {
            Value::F32(f) => Ok(*f),
            _ => Err(Error::InvalidConversion("f32", format!("{:?}", value))),
        }
    }
}
impl FromGlueSql for f64 {
    fn from_gluesql(value: &Value) -> Result<Self, Error> {
        match value {
            Value::F64(f) => Ok(*f),
            _ => Err(Error::InvalidConversion("f64", format!("{:?}", value))),
        }
    }
}
impl FromGlueSql for String {
    fn from_gluesql(value: &Value) -> Result<Self, Error> {
        match value {
            Value::Str(s) => Ok(s.clone()),
            _ => Err(Error::InvalidConversion("String", format!("{:?}", value))),
        }
    }
}
impl FromGlueSql for bool {
    fn from_gluesql(value: &Value) -> Result<Self, Error> {
        match value {
            Value::Bool(b) => Ok(*b),
            _ => Err(Error::InvalidConversion("bool", format!("{:?}", value))),
        }
    }
}

impl<T: FromGlueSql> FromGlueSql for Option<T> {
    fn from_gluesql(value: &Value) -> Result<Self, Error> {
        match value {
            Value::Null => Ok(None),
            _ => Ok(Some(T::from_gluesql(value)?)),
        }
    }
}
