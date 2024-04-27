pub trait ReflectGlueSql {
    fn reflect_gluesql_type() -> String;
    fn reflect_gluesql_type_with_nullability() -> String;
}
// https://gluesql.org/docs/0.15/sql-syntax/data-types/
// BOOLEAN
// Integer Types
// INT8: 8-bit signed integer
// INT16: 16-bit signed integer
// INT32: 32-bit signed integer
// INT or INTEGER: 64-bit signed integer (default)
// INT128: 128-bit signed integer
// UINT8: 8-bit unsigned integer
// UINT16: 16-bit unsigned integer
// UINT32: 32-bit unsigned integer
// UINT64: 64-bit unsigned integer
// UINT128: 128-bit unsigned integer
// FLOAT
// TEXT
// DECIMAL
// DATE
// TIMESTAMP
// TIME
// INTERVAL
// LIST
// MAP
// BYTEA
// INET
// UUID
impl ReflectGlueSql for bool {
    fn reflect_gluesql_type() -> String {
        "BOOLEAN".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "BOOLEAN NOT NULL".to_string()
    }
}
impl ReflectGlueSql for i8 {
    fn reflect_gluesql_type() -> String {
        "INT8".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "INT8 NOT NULL".to_string()
    }
}
impl ReflectGlueSql for i16 {
    fn reflect_gluesql_type() -> String {
        "INT16".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "INT16 NOT NULL".to_string()
    }
}
impl ReflectGlueSql for i32 {
    fn reflect_gluesql_type() -> String {
        "INT32".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "INT32 NOT NULL".to_string()
    }
}
impl ReflectGlueSql for i64 {
    fn reflect_gluesql_type() -> String {
        "INT".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "INT NOT NULL".to_string()
    }
}
impl ReflectGlueSql for i128 {
    fn reflect_gluesql_type() -> String {
        "INT128".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "INT128 NOT NULL".to_string()
    }
}
impl ReflectGlueSql for u8 {
    fn reflect_gluesql_type() -> String {
        "UINT8".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "UINT8 NOT NULL".to_string()
    }
}
impl ReflectGlueSql for u16 {
    fn reflect_gluesql_type() -> String {
        "UINT16".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "UINT16 NOT NULL".to_string()
    }
}
impl ReflectGlueSql for u32 {
    fn reflect_gluesql_type() -> String {
        "UINT32".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "UINT32 NOT NULL".to_string()
    }
}
impl ReflectGlueSql for u64 {
    fn reflect_gluesql_type() -> String {
        "UINT64".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "UINT64 NOT NULL".to_string()
    }
}
impl ReflectGlueSql for u128 {
    fn reflect_gluesql_type() -> String {
        "UINT128".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "UINT128 NOT NULL".to_string()
    }
}
impl ReflectGlueSql for f32 {
    fn reflect_gluesql_type() -> String {
        "FLOAT".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "FLOAT NOT NULL".to_string()
    }
}
impl ReflectGlueSql for f64 {
    fn reflect_gluesql_type() -> String {
        "FLOAT".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "FLOAT NOT NULL".to_string()
    }
}
impl ReflectGlueSql for String {
    fn reflect_gluesql_type() -> String {
        "TEXT".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "TEXT NOT NULL".to_string()
    }
}
impl ReflectGlueSql for &str {
    fn reflect_gluesql_type() -> String {
        "TEXT".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "TEXT NOT NULL".to_string()
    }
}
impl ReflectGlueSql for rust_decimal::Decimal {
    fn reflect_gluesql_type() -> String {
        "DECIMAL".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "DECIMAL NOT NULL".to_string()
    }
}
impl ReflectGlueSql for chrono::NaiveDate {
    fn reflect_gluesql_type() -> String {
        "DATE".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "DATE NOT NULL".to_string()
    }
}
impl ReflectGlueSql for chrono::NaiveDateTime {
    fn reflect_gluesql_type() -> String {
        "TIMESTAMP".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "TIMESTAMP NOT NULL".to_string()
    }
}
impl ReflectGlueSql for chrono::NaiveTime {
    fn reflect_gluesql_type() -> String {
        "TIME".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "TIME NOT NULL".to_string()
    }
}
impl ReflectGlueSql for chrono::Duration {
    fn reflect_gluesql_type() -> String {
        "INTERVAL".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "INTERVAL NOT NULL".to_string()
    }
}
impl ReflectGlueSql for bytes::Bytes {
    fn reflect_gluesql_type() -> String {
        "BYTEA".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "BYTEA NOT NULL".to_string()
    }
}
impl ReflectGlueSql for std::net::IpAddr {
    fn reflect_gluesql_type() -> String {
        "INET".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "INET NOT NULL".to_string()
    }
}
impl ReflectGlueSql for uuid::Uuid {
    fn reflect_gluesql_type() -> String {
        "UUID".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "UUID NOT NULL".to_string()
    }
}
impl<T: ReflectGlueSql> ReflectGlueSql for Option<T> {
    fn reflect_gluesql_type() -> String {
        T::reflect_gluesql_type()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        format!("{} NULL", T::reflect_gluesql_type())
    }
}
impl<T: ReflectGlueSql> ReflectGlueSql for Vec<T> {
    fn reflect_gluesql_type() -> String {
        "LIST".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "LIST NOT NULL".to_string()
    }
}
impl<K: ReflectGlueSql, V: ReflectGlueSql> ReflectGlueSql for std::collections::HashMap<K, V> {
    fn reflect_gluesql_type() -> String {
        "MAP".to_string()
    }
    fn reflect_gluesql_type_with_nullability() -> String {
        "MAP NOT NULL".to_string()
    }
}
