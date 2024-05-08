use crate::domain::repositories::user::UserRepository;
use crate::domain::services::user::UserService;
use crate::infrastructure::external::apalis::init_apalis;
use crate::infrastructure::external::email::{init_sender, Sender};
use crate::infrastructure::external::scylla::connect_scylladb;
use crate::infrastructure::repositories::user::UserScyllaRepository;
use crate::infrastructure::services::user::UserServiceImpl;
use std::sync::Arc;
#[derive(Clone)]
pub struct Container {
    pub user_service: Arc<dyn UserService>,
}

impl Container {
    pub async fn new() -> Self {
        let scylla_session = connect_scylladb().await.unwrap();
        let email_sender: Sender = init_sender().await;
        let apalis = init_apalis().await.expect("failed to init apalis");
        let user_repo: Arc<dyn UserRepository> = Arc::new(UserScyllaRepository::new(
            Arc::new(scylla_session),
            Arc::new(email_sender),
            Arc::new(apalis),
        ));
        let user_service = Arc::new(UserServiceImpl {
            repository: user_repo,
        });
        Container { user_service }
    }
}
