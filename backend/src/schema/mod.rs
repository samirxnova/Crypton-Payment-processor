use async_graphql::{EmptySubscription, Schema, SimpleObject};
use bonsaidb::core::schema::{Collection, Schema as BonsaiSchema};
use serde::{Deserialize, Serialize};

pub mod mutation;
pub mod query;
pub use mutation::MutationRoot;
pub use query::QueryRoot;

pub type GraphqlSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

#[derive(Debug, BonsaiSchema)]
#[schema(name = "schema", collections = [User, Transaction])]
pub struct DbSchema;

#[derive(Debug, Serialize, Deserialize, Collection, Clone, SimpleObject)]
#[collection(name = "users")]
pub struct User {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Collection, Clone, SimpleObject)]
#[collection(name = "transactions")]
pub struct Transaction {
    pub id: String,
}
