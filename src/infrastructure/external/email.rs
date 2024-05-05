use dotenv::dotenv;
use lettre::transport::smtp::authentication::Credentials;
use lettre::SmtpTransport;
pub struct Sender {
    pub email: String,
    pub mailer: SmtpTransport,
}
pub async fn init_sender() -> Sender {
    dotenv().ok();
    let email = std::env::var("EMAIL").unwrap();
    let token = std::env::var("EMAIL_TOKEN").unwrap();
    let creds = Credentials::new(email.clone(), token);
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();
    Sender { email, mailer }
}
