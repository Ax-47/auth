use crate::domain::entities::user::CreateUser;

use async_trait::async_trait;
use scylla::transport::errors::QueryError;
use scylla::QueryResult;
#[async_trait]
pub trait UserService: Sync + Send {
    async fn create(&self, new_user: CreateUser) -> Result<QueryResult, QueryError>;
}
