use gluesql_derive::ToGlueSqlRow;
use std::str::FromStr;

#[test]
fn test_togluesql_field_struct() {
    #[derive(ToGlueSqlRow)]
    struct Foo {
        a: i64,
        b: bool,
        c: String,
        d: Option<i64>,
        e: rust_decimal::Decimal,
    }
    let data = Foo {
        a: 1,
        b: true,
        c: "hello".to_string(),
        d: None,
        e: rust_decimal::Decimal::from_str("1.23").unwrap(),
    };
    let row = data.to_gluesql_row();
    println!("{:?}", row);
}
