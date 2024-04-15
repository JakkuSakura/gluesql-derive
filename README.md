# gluesql-derive

generate traits like ReflectGlueSqlRow, FromGlueSqlRow, ToGlueSqlRow to enable basic ORM functionality

# Example

```rust
#[test]
fn test_reflectgluesql_field_struct() {
    #[allow(unused)]
    #[derive(ReflectGlueSqlRow)]
    struct Foo {
        a: i64,
        b: bool,
        c: String,
        d: Option<i64>,
    }
    assert_eq!(Foo::columns(), vec!["a", "b", "c", "d"]);
}

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
        &[
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ],
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

```