use gluesql_core::ast_builder::{date, expr, null, num, text, timestamp, ExprNode};

pub trait ToGlueSql {
    fn to_gluesql(&self) -> ExprNode<'static>;
}

impl ToGlueSql for String {
    fn to_gluesql(&self) -> ExprNode<'static> {
        text(self.clone())
    }
}
impl ToGlueSql for &str {
    fn to_gluesql(&self) -> ExprNode<'static> {
        text(self.to_string())
    }
}
impl ToGlueSql for f64 {
    fn to_gluesql(&self) -> ExprNode<'static> {
        num(*self)
    }
}
impl ToGlueSql for f32 {
    fn to_gluesql(&self) -> ExprNode<'static> {
        num(*self)
    }
}
impl ToGlueSql for i8 {
    fn to_gluesql(&self) -> ExprNode<'static> {
        num(*self)
    }
}
impl ToGlueSql for i16 {
    fn to_gluesql(&self) -> ExprNode<'static> {
        num(*self)
    }
}
impl ToGlueSql for i32 {
    fn to_gluesql(&self) -> ExprNode<'static> {
        num(*self)
    }
}
impl ToGlueSql for i64 {
    fn to_gluesql(&self) -> ExprNode<'static> {
        num(*self)
    }
}
impl ToGlueSql for u8 {
    fn to_gluesql(&self) -> ExprNode<'static> {
        num(*self)
    }
}
impl ToGlueSql for u16 {
    fn to_gluesql(&self) -> ExprNode<'static> {
        num(*self)
    }
}
impl ToGlueSql for u32 {
    fn to_gluesql(&self) -> ExprNode<'static> {
        num(*self)
    }
}
impl ToGlueSql for u64 {
    fn to_gluesql(&self) -> ExprNode<'static> {
        num(*self)
    }
}
impl ToGlueSql for bool {
    fn to_gluesql(&self) -> ExprNode<'static> {
        (*self).into()
    }
}
impl<T: ToGlueSql> ToGlueSql for Option<T> {
    fn to_gluesql(&self) -> ExprNode<'static> {
        match self {
            Some(v) => v.to_gluesql(),
            None => null(),
        }
    }
}
impl ToGlueSql for () {
    fn to_gluesql(&self) -> ExprNode<'static> {
        null()
    }
}
impl ToGlueSql for rust_decimal::Decimal {
    fn to_gluesql(&self) -> ExprNode<'static> {
        expr(self.to_string())
    }
}
impl ToGlueSql for chrono::DateTime<chrono::Utc> {
    fn to_gluesql(&self) -> ExprNode<'static> {
        timestamp(self.to_rfc3339())
    }
}
impl ToGlueSql for chrono::NaiveDateTime {
    fn to_gluesql(&self) -> ExprNode<'static> {
        num(self.and_utc().timestamp_micros())
    }
}
impl ToGlueSql for chrono::NaiveTime {
    fn to_gluesql(&self) -> ExprNode<'static> {
        date(self.to_string())
    }
}

impl ToGlueSql for chrono::NaiveDate {
    fn to_gluesql(&self) -> ExprNode<'static> {
        date(self.to_string())
    }
}
impl ToGlueSql for chrono::Duration {
    fn to_gluesql(&self) -> ExprNode<'static> {
        num(self.num_microseconds().unwrap())
    }
}
impl ToGlueSql for std::time::Duration {
    fn to_gluesql(&self) -> ExprNode<'static> {
        num(self.as_micros() as i64)
    }
}
impl ToGlueSql for std::time::SystemTime {
    fn to_gluesql(&self) -> ExprNode<'static> {
        num(self
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros() as i64)
    }
}

impl ToGlueSql for uuid::Uuid {
    fn to_gluesql(&self) -> ExprNode<'static> {
        text(self.to_string())
    }
}
impl ToGlueSql for std::net::IpAddr {
    fn to_gluesql(&self) -> ExprNode<'static> {
        text(self.to_string())
    }
}
