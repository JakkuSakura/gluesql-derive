use gluesql_core::data::Value;
use gluesql_derive::FromGlueSqlRow;

#[test]
fn test_fromgluesql_field_struct() {
    #[derive(FromGlueSqlRow)]
    struct Foo {
        a: i64,
        b: bool,
        c: String,
        d: Option<i64>,
    }
    let data = Foo::from_gluesql_row(
        &["a".to_string(), "b".to_string(), "c".to_string()],
        vec![
            Value::I64(1),
            Value::Bool(true),
            Value::Str("hello".to_string()),
            Value::Null,
        ],
    )
    .unwrap();
    assert_eq!(data.a, 1);
    assert_eq!(data.b, true);
    assert_eq!(data.c, "hello");
    assert_eq!(data.d, None);
}
