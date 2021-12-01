use proc_macro_function_like::*;

#[macro_use]
extern crate const_format;
mod helper_traits;

fn main() {
    println!("Hello, world!");
}

// #[tokio::test]
async fn test() {
    use serde::{Deserialize, Serialize};
    use sqlx::FromRow;

    #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
    pub struct User {
        pub id: i32,
        pub login: String,
        pub email: Option<String>,
    }

    impl User {
        pub const TABLE_NAME: &'static str = "users";
    }

    let dsn = "postgres://user:password@localhost:5432/postgres_db";
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(25)
        .min_connections(5)
        .connect_timeout(tokio::time::Duration::from_secs(10))
        .connect(dsn)
        .await
        .unwrap();

    let mut executor = pool.acquire().await.unwrap();
    let login_var = "my_login".to_string();
    let email_var = "example@example.com".to_string();

    let s = pg_query!(User, &mut executor, login = login_var, hourly_rate = Some(10));
    let s2 = pg_query!(User, &mut executor, login = "login", hourly_rate = Some(10.0));
    let s3 = pg_query!(User, &mut executor, login = login_var, hourly_rate = unsafe {*(10.0_f32 as const* f32)});
    let s4 = pg_query_broken!(User, &mut executor, login > login_var, hourly_rate = Some(10));
    let s5 = pg_query_broken!(User, &mut executor, login = 123, hourly_rate = Some(10.0));
    let s6 = pg_query_broken!(User, &mut executor, login => "login", hourly_rate = Some(10.0));
}
