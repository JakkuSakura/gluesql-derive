use chrono::Utc;
use gluesql_core::ast_builder;
use gluesql_core::ast_builder::Build;
use gluesql_core::prelude::{Glue, Payload};
use gluesql_shared_memory_storage::SharedMemoryStorage;

use gluesql_derive::{
    FromGlueSql, FromGlueSqlRow, ReflectGlueSql, ReflectGlueSqlRow, ToGlueSql, ToGlueSqlRow,
};

async fn test_type<T>(val: T) -> eyre::Result<()>
where
    T: PartialEq + std::fmt::Debug + ReflectGlueSql + FromGlueSql + ToGlueSql,
{
    #[derive(Debug, PartialEq, ReflectGlueSqlRow, FromGlueSqlRow, ToGlueSqlRow)]
    struct Foo<T> {
        foo: T,
    }
    let db = SharedMemoryStorage::new();
    let mut glue = Glue::new(db);
    let ddl = Foo::<T>::get_ddl("foo");
    glue.execute(ddl.as_str()).await?;
    let foo = Foo::<T> { foo: val };
    let insert = ast_builder::table("foo")
        .insert()
        .columns(Foo::<T>::columns())
        .values(vec![foo.to_gluesql_row()])
        .build()?;
    glue.execute_stmt(&insert).await?;
    let select = ast_builder::table("foo")
        .select()
        .project(Foo::<T>::columns())
        .build()
        .unwrap();
    let result = glue.execute_stmt(&select).await?;
    match result {
        Payload::Select { labels, rows } => {
            let row = Foo::from_gluesql_rows(&labels, rows)?;
            assert_eq!(row[0], foo);
        }
        _ => panic!("unexpected result"),
    }
    Ok(())
}

#[tokio::test]
async fn test_datetime() -> eyre::Result<()> {
    test_type(Utc::now()).await
}

#[tokio::test]
async fn test_time() -> eyre::Result<()> {
    test_type(Utc::now().naive_utc()).await
}

#[tokio::test]
async fn test_duration() -> eyre::Result<()> {
    test_type(chrono::Duration::seconds(1)).await
}
#[tokio::test]
async fn test_std_duration() -> eyre::Result<()> {
    test_type(std::time::Duration::from_secs(1)).await
}
