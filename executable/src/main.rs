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
    let s2 = pg_query!(User, &mut executor, login = login_var, hourly_rate = 10.0);
    let s3 = pg_query!(User, &mut executor, login = login_var, hourly_rate = Some(10.0));
    // dbg!(s);


    let f = sqlx::query_as!(
        User,
        r#"SELECT id, avatar_url, login, email, firstname, lastname, description, hourly_rate,
        currency, created_at, state as "state: _", role as "role: _" FROM users WHERE login = $1 AND hourly_rate = $2"#,
        "rest",
        Some(10.0_f64)
    )
        .fetch_optional(&mut executor)
        .await.unwrap();
    dbg!(f);

    let f2 = sqlx::query_as!(
        User,
        r#"SELECT id, avatar_url, login, email, firstname, lastname, description, hourly_rate,
        currency, created_at, state as "state: _", role as "role: _" FROM users WHERE login = $1 AND hourly_rate = $2"#,
        "rest",
        10.0_f64
    )
        .fetch_optional(&mut executor)
        .await.unwrap();
    dbg!(f2);

    let f3 = sqlx::query_as!(
        User,
        r#"SELECT id, avatar_url, login, email, firstname, lastname, description, hourly_rate,
        currency, created_at, state as "state: _", role as "role: _" FROM users WHERE login = $1 AND hourly_rate = $2"#,
        "rest",
        10_f64
    )
        .fetch_optional(&mut executor)
        .await.unwrap();
    dbg!(f3);

    let f4 = sqlx::query_as!(
        User,
        r#"SELECT id, avatar_url, login, email, firstname, lastname, description, hourly_rate, currency, created_at,
        state as "state: _", role as "role: _" FROM users WHERE login = $1 AND email = $2"#,
        "rest",
        "123"
    )
        .fetch_optional(&mut executor)
        .await.unwrap();
    dbg!(f4);
}
