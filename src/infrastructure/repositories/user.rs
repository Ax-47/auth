use crate::domain::entities::confirm;
use crate::domain::entities::user::CreateUser;
use crate::domain::repositories::user::UserRepository;
use crate::infrastructure::external::email::Sender;
use apalis::redis::RedisStorage;
use async_trait::async_trait;
use lettre::message::header::ContentType;
use lettre::transport::smtp::Error as SmtpError;
use lettre::{Message, Transport};
use liquid::ParserBuilder;
use scylla::transport::errors::QueryError;
use scylla::{QueryResult, Session};
use std::sync::Arc;
pub struct UserScyllaRepository {
    pub scylla_session: Arc<Session>,
    pub emailer: Arc<Sender>,
    pub storage: Arc<RedisStorage<confirm::Confirm>>,
}

impl UserScyllaRepository {
    pub fn new(
        session: Arc<Session>,
        credentials: Arc<Sender>,
        storage: Arc<RedisStorage<confirm::Confirm>>,
    ) -> Self {
        UserScyllaRepository {
            scylla_session: session,
            emailer: credentials,
            storage,
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
    async fn create_confirm(
        &self,
        id: uuid::Uuid,
        email: String,
    ) -> Result<QueryResult, QueryError> {
        let query_result = self
            .scylla_session
            .query(
                "INSERT INTO auth.confirm (id,email) VALUES(?,?)",
                (id, email),
            )
            .await?;
        Ok(query_result)
    }
    async fn find_user_by_email(&self, email: String) -> Result<QueryResult, QueryError> {
        let query_result = self
            .scylla_session
            .query("select * from auth.users where email = ?", [email].as_ref())
            .await?;
        Ok(query_result)
    }

    async fn is_email_exist(&self, email: String) -> Result<bool, QueryError> {
        let query_result = self
            .scylla_session
            .query(
                "select id from auth.users where email = ?",
                [email].as_ref(),
            )
            .await?
            .rows;
        if let Some(rows) = query_result {
            return Ok(rows.last().is_some());
        }
        Ok(false)
    }

    async fn send_confirm_email(&self, email: String, confirm_id: String) -> Result<(), SmtpError> {
        let mailer = self.emailer.mailer.clone();
        let email_sender = self.emailer.email.clone();
        println!("test");
        let source = "<a href=\"127.0.0.1:8080/auth/{{ confirm_id }}/sign_up\">click here</a>";
        let template = ParserBuilder::with_stdlib()
            .build()
            .unwrap()
            .parse(source)
            .unwrap();
        let globals = liquid::object!({
        "confirm_id":confirm_id,
               });

        println!("test2");
        let email_b = Message::builder()
            .from(email_sender.parse().unwrap())
            .to(email.parse().unwrap())
            .subject("Sign Up")
            .header(ContentType::TEXT_HTML)
            .body(template.render(&globals).unwrap())
            .unwrap();

        println!("test3");
        mailer.send(&email_b)?;
        Ok(())
    }
    async fn get_storage(&self) -> Arc<RedisStorage<confirm::Confirm>> {
        self.storage.clone()
    }
}
