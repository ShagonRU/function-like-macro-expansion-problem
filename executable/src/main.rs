use proc_macro_function_like::*;

#[macro_use]
extern crate const_format;
mod helper_traits;

#[tokio::main]
async fn main() {
    let dsn_1 = "postgres://sqlx_user:sqlx_pass@localhost:5656/sqlx_work_please";
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(25)
        .min_connections(5)
        .connect_timeout(tokio::time::Duration::from_secs(10))
        .connect(dsn_1)
        .await
        .unwrap();

    let mut _executor_1 = pool.acquire().await.unwrap();

    //

    let dsn_2 = "postgresql://";
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(25)
        .min_connections(5)
        .connect_timeout(tokio::time::Duration::from_secs(10))
        .connect(dsn_2)
        .await
        .unwrap();

    let mut _executor_2 = pool.acquire().await.unwrap();

    //

    let dsn_3 = "postgresql://12312;fads;afdsv";
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(25)
        .min_connections(5)
        .connect_timeout(tokio::time::Duration::from_secs(10))
        .connect(dsn_3)
        .await
        .unwrap();

    let mut _executor_3 = pool.acquire().await.unwrap();
}
