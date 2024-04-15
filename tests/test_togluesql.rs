use gluesql_derive::ToGlueSqlRow;

#[test]
fn test_togluesql_field_struct() {
    #[derive(ToGlueSqlRow)]
    struct Foo {
        a: i64,
        b: bool,
        c: String,
        d: Option<i64>,
    }
    let data = Foo {
        a: 1,
        b: true,
        c: "hello".to_string(),
        d: None,
    };
    let row = data.to_gluesql_row();
    println!("{:?}", row);
}
