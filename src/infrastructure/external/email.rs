use lettre::transport::smtp::authentication::Credentials;

pub async fn init_sender() -> Credentials {
    let email = std::env::var("EMAIL").unwrap();
    let token = std::env::var("EMAIL_TOKEN").unwrap();
    Credentials::new(email, token)
}
