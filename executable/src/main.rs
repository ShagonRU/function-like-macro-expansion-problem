mod helper_traits;

#[tokio::main]
async fn main() {
    let dsn_3 = "postgresql://";
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(25)
        .min_connections(5)
        .connect_timeout(tokio::time::Duration::from_secs(10))
        .connect(dsn_3)
        .await
        .unwrap();

    let mut _executor_3 = pool.acquire().await.unwrap();
}
