use scylla::{Session, SessionBuilder};
use std::error::Error;
pub async fn connect_scylladb() -> Result<Session, Box<dyn Error>> {
    let uri = std::env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0.1:9042".to_string());
    let session: Session = SessionBuilder::new().known_node(uri).build().await?;
    Ok(session)
}
