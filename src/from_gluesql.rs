use std::any::type_name;
use std::collections::HashMap;
use std::str::FromStr;

use chrono::{TimeZone, Utc};
use gluesql_core::data::{Interval, Value};

use crate::Error;

pub trait FromGlueSql: Sized {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error>;
}

impl FromGlueSql for i8 {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::I8(i) => Ok(i),
            _ => Err(Error::InvalidConversion("i8", value)),
        }
    }
}
impl FromGlueSql for i16 {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::I16(i) => Ok(i),
            _ => Err(Error::InvalidConversion("i16", value)),
        }
    }
}
impl FromGlueSql for i32 {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::I32(i) => Ok(i),
            _ => Err(Error::InvalidConversion("i32", value)),
        }
    }
}
impl FromGlueSql for i64 {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::I64(i) => Ok(i),
            _ => Err(Error::InvalidConversion("i64", value)),
        }
    }
}
impl FromGlueSql for i128 {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::I128(i) => Ok(i),
            _ => Err(Error::InvalidConversion("i128", value)),
        }
    }
}
impl FromGlueSql for u8 {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::U8(i) => Ok(i),
            _ => Err(Error::InvalidConversion("u8", value)),
        }
    }
}
impl FromGlueSql for u16 {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::U16(i) => Ok(i),
            _ => Err(Error::InvalidConversion("u16", value)),
        }
    }
}
impl FromGlueSql for u32 {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::U32(i) => Ok(i),
            _ => Err(Error::InvalidConversion("u32", value)),
        }
    }
}
impl FromGlueSql for u64 {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::U64(i) => Ok(i),
            _ => Err(Error::InvalidConversion("u64", value)),
        }
    }
}
impl FromGlueSql for u128 {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::U128(i) => Ok(i),
            _ => Err(Error::InvalidConversion("u128", value)),
        }
    }
}
impl FromGlueSql for f32 {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::F32(f) => Ok(f),
            _ => Err(Error::InvalidConversion("f32", value)),
        }
    }
}
impl FromGlueSql for f64 {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::F64(f) => Ok(f),
            _ => Err(Error::InvalidConversion("f64", value)),
        }
    }
}
impl FromGlueSql for String {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::Str(s) => Ok(s.clone()),
            _ => Err(Error::InvalidConversion("String", value)),
        }
    }
}
impl FromGlueSql for bool {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::Bool(b) => Ok(b),
            _ => Err(Error::InvalidConversion("bool", value)),
        }
    }
}

impl<T: FromGlueSql> FromGlueSql for Option<T> {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::Null => Ok(None),
            _ => Ok(Some(T::from_gluesql(value)?)),
        }
    }
}
impl FromGlueSql for bytes::Bytes {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::Bytea(b) => Ok(b.into()),
            _ => Err(Error::InvalidConversion("Vec<u8>", value)),
        }
    }
}
impl FromGlueSql for rust_decimal::Decimal {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::Decimal(d) => Ok(d),
            _ => Err(Error::InvalidConversion("Decimal", value)),
        }
    }
}
impl FromGlueSql for chrono::NaiveDate {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::Str(s) => chrono::NaiveDate::from_str(&s)
                .map_err(|e| Error::InvalidConversion("NaiveDate", Value::Str(s))),
            Value::Date(d) => Ok(d),
            _ => Err(Error::InvalidConversion("NaiveDate", value)),
        }
    }
}
impl FromGlueSql for chrono::NaiveDateTime {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::I64(i) => Utc
                .timestamp_micros(i)
                .single()
                .map(|d| d.naive_utc())
                .ok_or(Error::InvalidConversion("NaiveDateTime", value)),
            Value::Timestamp(d) => Ok(d),
            _ => Err(Error::InvalidConversion("NaiveDateTime", value)),
        }
    }
}
impl FromGlueSql for chrono::DateTime<Utc> {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::I64(i) => Utc
                .timestamp_micros(i)
                .single()
                .ok_or(Error::InvalidConversion("DateTime<Utc>", value)),
            Value::Time(d) => Ok(d.into()),
            // There is no Value::TimestampTz
            Value::Timestamp(d) => Ok(d.and_utc()),
            _ => Err(Error::InvalidConversion("DateTime<Utc>", value)),
        }
    }
}
impl FromGlueSql for chrono::Duration {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::I64(m) => Ok(chrono::Duration::microseconds(m)),
            Value::Interval(Interval::Microsecond(m)) => Ok(chrono::Duration::microseconds(m)),
            _ => Err(Error::InvalidConversion("chrono::Duration", value)),
        }
    }
}
impl FromGlueSql for std::time::Duration {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::I64(m) => Ok(std::time::Duration::from_micros(m as u64)),
            Value::Interval(Interval::Microsecond(m)) => {
                Ok(std::time::Duration::from_micros(m as u64))
            }
            _ => Err(Error::InvalidConversion("std::time::Duration", value)),
        }
    }
}
impl FromGlueSql for uuid::Uuid {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::Uuid(uuid) => {
                let uuid = uuid.to_be_bytes();
                Ok(uuid::Uuid::from_slice(&uuid).unwrap())
            }
            _ => Err(Error::InvalidConversion("Uuid", value)),
        }
    }
}
impl FromGlueSql for std::net::IpAddr {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::Inet(ip) => Ok(ip),
            _ => Err(Error::InvalidConversion("IpAddr", value)),
        }
    }
}
impl<T: FromGlueSql> FromGlueSql for HashMap<String, T> {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::Map(map) => {
                let mut result = HashMap::new();
                for (k, v) in map {
                    result.insert(k, T::from_gluesql(v)?);
                }
                Ok(result)
            }
            _ => Err(Error::InvalidConversion(
                type_name::<HashMap<String, T>>(),
                value,
            )),
        }
    }
}
impl<T: FromGlueSql> FromGlueSql for Vec<T> {
    fn from_gluesql(value: Value) -> gluesql_core::error::Result<Self, Error> {
        match value {
            Value::List(list) => {
                let mut result = Vec::new();
                for v in list {
                    result.push(T::from_gluesql(v)?);
                }
                Ok(result)
            }
            _ => Err(Error::InvalidConversion(type_name::<Vec<T>>(), value)),
        }
    }
}
