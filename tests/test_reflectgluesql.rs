use gluesql_derive::ReflectGlueSqlRow;

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
