use sqlx::postgres::PgArguments;
use sqlx::{Arguments, Database, Encode, Postgres, Type};

pub trait ChainedArguments<'q>: Send + Sized + Default {
    type Database: Database;

    /// Add the value to the end of the arguments and return Self.
    fn add_c<T>(self, value: T) -> Self
        where
            T: 'q + Send + Encode<'q, Self::Database> + Type<Self::Database>;
}

impl<'q> ChainedArguments<'q> for PgArguments {
    type Database = Postgres;

    /// Chained version of common ['Arguments::add()'] associated method.
    /// Always return Self.
    /// ```edition2018
    /// pub async fn some_fn<'a>(pool: &PgPool) {
    ///     let res = sqlx::query_with::<_, _>(
    ///         "SELECT $1",
    ///         PgArguments::default().add_c(1)
    ///     )
    ///         .fetch_one(pool).await.unwrap();
    ///
    ///     println!("{:?}", res.get::<i32, _>(0));
    /// }
    /// ```
    fn add_c<T: 'q>(mut self, value: T) -> Self
        where
            T: 'q + Send + Encode<'q, Self::Database> + Type<Self::Database>,
    {
        self.add(value);
        self
    }
}
