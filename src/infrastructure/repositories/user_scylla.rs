use crate::domain::entities::user::CreateUser;
use crate::domain::repositories::user::UserRepository;
use async_trait::async_trait;
use scylla::transport::errors::QueryError;
use scylla::{QueryResult, Session};
use std::sync::Arc;
pub struct UserScyllaRepository {
    pub scylla_session: Arc<Session>,
}

impl UserScyllaRepository {
    pub fn new(session: Arc<Session>) -> Self {
        UserScyllaRepository {
            scylla_session: session,
        }
    }
}
#[async_trait]
impl UserRepository for UserScyllaRepository {
    async fn create(&self, new_user: &CreateUser) -> Result<QueryResult, QueryError> {
        let query_result = self
            .scylla_session
            .query(
                "INSERT INTO auth.users (name,tag,email,password,role) VALUES(?,?,?,?,?)",
                new_user,
            )
            .await?;
        Ok(query_result)
    }
}
