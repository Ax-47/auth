use crate::domain::entities::user::CreateUser;
use crate::domain::repositories::user::UserRepository;
use async_trait::async_trait;
use lettre::transport::smtp::authentication::Credentials;
use scylla::transport::errors::QueryError;
use scylla::{QueryResult, Session};
use std::sync::Arc;
pub struct UserScyllaRepository {
    pub scylla_session: Arc<Session>,
    pub email_credentials: Arc<Credentials>,
}

impl UserScyllaRepository {
    pub fn new(session: Arc<Session>, credentials: Arc<Credentials>) -> Self {
        UserScyllaRepository {
            scylla_session: session,
            email_credentials: credentials,
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
    async fn find_user_by_email(&self, email: String) -> Result<QueryResult, QueryError> {
        let query_result = self
            .scylla_session
            .query("select * from auth.user where email = ?", [email].as_ref())
            .await?;
        Ok(query_result)
    }

    async fn is_email_exist(&self, email: String) -> Result<bool, QueryError> {
        let query_result = self
            .scylla_session
            .query("select id from auth.user where email = ?", [email].as_ref())
            .await?
            .rows;
        if query_result.is_some() {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
