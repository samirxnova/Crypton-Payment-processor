use super::{Transaction, User};
use async_graphql::{Context, InputObject, Object, Result};
use ulid::Ulid;

#[derive(InputObject, Debug)]
struct NewUser {
    id: String,
}

#[derive(InputObject, Debug)]
struct NewTransaction {
    id: String,
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_user<'ctx>(
        &self,
        _context: &Context<'ctx>,
        _new_user: NewUser,
    ) -> Result<User> {
        let id = Ulid::new().to_string();
        Ok(User { id })
    }

    async fn create_transaction<'ctx>(
        &self,
        _context: &Context<'ctx>,
        _new_transaction: NewTransaction,
    ) -> Result<Transaction> {
        let id = Ulid::new().to_string();
        Ok(Transaction { id })
    }
}
