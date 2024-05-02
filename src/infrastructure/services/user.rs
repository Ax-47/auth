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
}
