use async_graphql::{Context, Object, Result};
use bonsaidb::local::AsyncDatabase;

use super::{Transaction, User};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn users<'ctx>(&self, context: &Context<'ctx>) -> Result<Vec<User>> {
        let _db = context
            .data::<AsyncDatabase>()
            .expect("failed to get user database");
        // db.0.get("id").await?;
        Ok(vec![User {
            id: String::from("nothing"),
        }])
    }

    async fn transactions<'ctx>(&self, _context: &Context<'ctx>) -> Result<Vec<Transaction>> {
        // let pool = context
        //     .data::<SqlitePool>()
        //     .expect("failed to get connection pool");
        // // TODO find a way to type check queries with an enum in the struct
        // let agents = sqlx::query_as::<_, Agent>("SELECT * FROM agents ORDER BY inserted_at DESC")
        //     .fetch_all(pool)
        //     .await?;
        Ok(vec![Transaction {
            id: String::from("nothing"),
        }])
    }
}
