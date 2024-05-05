use crate::domain::entities::user::CreateUser;
use crate::domain::repositories::user::UserRepository;
use crate::infrastructure::external::email::Sender;
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
}

impl UserScyllaRepository {
    pub fn new(session: Arc<Session>, credentials: Arc<Sender>) -> Self {
        UserScyllaRepository {
            scylla_session: session,
            emailer: credentials,
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
        Ok(query_result.is_some())
    }

    async fn send_confirm_email(&self, email: String, confirm_id: String) -> Result<(), SmtpError> {
        let mailer = self.emailer.mailer.clone();
        let email_sender = self.emailer.email.clone();
        let source =
            "<a href=\"127.0.0.1:8080/2fa/{{ confirm_id }}/sign_up\">click fucking here</a>";
        let template = ParserBuilder::with_stdlib()
            .build()
            .unwrap()
            .parse(source)
            .unwrap();
        let globals = liquid::object!({
        "confirm_id":confirm_id,
               });
        let email_b = Message::builder()
            .from(email_sender.parse().unwrap())
            .to(email.parse().unwrap())
            .subject("Sign Up")
            .header(ContentType::TEXT_HTML)
            .body(template.render(&globals).unwrap())
            .unwrap();

        mailer.send(&email_b)?;
        Ok(())
    }
}
