mod helper_traits;

#[tokio::main]
async fn main() {

    mod case_1 {
        use sqlx::Row;

        type Conn = sqlx::pool::PoolConnection<sqlx::Postgres>;

        async fn by_mut_ref(executor: &mut Conn) -> i32 {
            let res1 = sqlx::query("SELECT 1").fetch_one(executor).await.unwrap().get::<i32, _>(0);
            let res2 = sqlx::query("SELECT 1").fetch_one(executor).await.unwrap().get::<i32, _>(0);
            res1 + res2
        }

        async fn mut_move(mut executor: Conn) -> i32 {
            let res1 = sqlx::query("SELECT 1").fetch_one(&mut executor).await.unwrap().get::<i32, _>(0);
            let res2 = sqlx::query("SELECT 1").fetch_one(&mut executor).await.unwrap().get::<i32, _>(0);
            res1 + res2
        }

        async fn some_func_mut(mut executor: Conn) -> i32 {
            let res3 = mut_move(executor).await;
            let res1 = by_mut_ref(&mut executor).await;
            let res2 = by_mut_ref(&mut executor).await;
            let res4 = mut_move(executor).await;
            res1 + res2 + res3 + res4
        }

        async fn some_func_mut_ref(mut executor: &mut Conn) -> i32 {
            let res3 = mut_move(executor).await;
            let res1 = by_mut_ref(executor).await;
            let res2 = by_mut_ref(executor).await;
            let res4 = mut_move(executor).await;
            res1 + res2 + res3 + res4
        }
    }
}
