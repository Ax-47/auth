use std::sync::Arc;

use crate::domain::entities::user::CreateUser;
use crate::domain::repositories::user::UserRepository;
use crate::domain::services::user::UserService;
use async_trait::async_trait;
use scylla::transport::errors::QueryError;
use scylla::QueryResult;
#[derive(Clone)]
pub struct UserServiceImpl {
    pub repository: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        UserServiceImpl { repository }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create(&self, user: CreateUser) -> Result<QueryResult, QueryError> {
        let mut cloned = user.clone();
        Ok(self.repository.create(&mut cloned).await?)
    }
    async fn find_user_by_email(&self, email: String) -> Result<QueryResult, QueryError> {
        Ok(self.repository.find_user_by_email(email.clone()).await?)
    }
    async fn is_email_exist(&self, email: String) -> Result<bool, QueryError> {
        Ok(self.repository.is_email_exist(email.clone()).await?)
    }
}
