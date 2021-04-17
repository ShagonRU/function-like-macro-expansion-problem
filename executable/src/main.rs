mod helper_traits;

#[tokio::main]
async fn main() {

    mod case_1 {
        type Conn = sqlx::pool::PoolConnection<sqlx::Postgres>;

        fn by_mut_ref(executor: &mut Conn) -> i32 {
            let res1 = sqlx::query_as::<_, i32>("SELECT 1").fetch_one(executor).await.unwrap();
            let res2 = sqlx::query_as::<_, i32>("SELECT 1").fetch_one(executor).await.unwrap();
            res1 + res2
        }

        fn mut_move(mut executor: Conn) -> i32 {
            let res1 = sqlx::query_as::<_, i32>("SELECT 1").fetch_one(&mut executor).await.unwrap();
            let res2 = sqlx::query_as::<_, i32>("SELECT 1").fetch_one(&mut executor).await.unwrap();
            res1 + res2
        }

        fn some_func_mut(mut executor: Conn) -> i32 {
            let res3 = mut_move(executor);
            let res1 = by_mut_ref(&mut executor);
            let res2 = by_mut_ref(&mut executor);
            let res4 = mut_move(executor);
            res1 + res2 + res3 + res4
        }

        fn some_func_mut_ref(mut executor: &mut Conn) -> i32 {
            let res3 = mut_move(executor);
            let res1 = by_mut_ref(executor);
            let res2 = by_mut_ref(executor);
            let res4 = mut_move(executor);
            res1 + res2 + res3 + res4
        }
    }
}
