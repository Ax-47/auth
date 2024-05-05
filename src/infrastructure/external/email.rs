use dotenv::dotenv;
use lettre::transport::smtp::authentication::Credentials;
pub struct Sender {
    pub email: String,
    pub creds: Credentials,
}
pub async fn init_sender() -> Sender {
    dotenv().ok();
    let email = std::env::var("EMAIL").unwrap();
    let token = std::env::var("EMAIL_TOKEN").unwrap();
    let creds = Credentials::new(email.clone(), token);
    Sender { email, creds }
}
