use crate::domain::entities::user::CreateUser;

use async_trait::async_trait;
use scylla::transport::errors::QueryError;
use scylla::QueryResult;
#[async_trait]
pub trait UserService: Sync + Send {
    async fn create(&self, new_user: CreateUser) -> Result<QueryResult, QueryError>;
    async fn find_user_by_email(&self, email: String) -> Result<QueryResult, QueryError>;
    async fn is_email_exist(&self, email: String) -> Result<bool, QueryError>;
}
