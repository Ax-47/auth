use crate::domain::entities::user::CreateUser;

use apalis::redis::RedisStorage;

use crate::domain::entities::confirm;
use async_trait::async_trait;
use lettre::transport::smtp::Error;
use scylla::transport::errors::QueryError;
use scylla::QueryResult;
#[async_trait]
pub trait UserService: Sync + Send {
    async fn create(&self, new_user: CreateUser) -> Result<QueryResult, QueryError>;
    async fn create_confirm(
        &self,
        id: uuid::Uuid,
        email: String,
    ) -> Result<QueryResult, QueryError>;
    async fn find_user_by_email(&self, email: String) -> Result<QueryResult, QueryError>;
    async fn is_email_exist(&self, email: String) -> Result<bool, QueryError>;
    async fn send_confirm_email(&self, email: String, confirm_id: String) -> Result<(), Error>;

    async fn get_storage(&self) -> RedisStorage<confirm::Confirm>;
}
