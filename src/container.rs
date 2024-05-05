use crate::domain::repositories::user::UserRepository;
use crate::domain::services::user::UserService;
use crate::infrastructure::external::scylla::connect_scylladb;
use crate::infrastructure::repositories::user_scylla::UserScyllaRepository;
use crate::infrastructure::services::user::UserServiceImpl;
use std::sync::Arc;
#[derive(Clone)]
pub struct Container {
    pub user_service: Arc<dyn UserService>,
}

impl Container {
    pub async fn new() -> Self {
        let scylla_session = connect_scylladb().await.unwrap();
        let user_repo: Arc<dyn UserRepository> =
            Arc::new(UserScyllaRepository::new(Arc::new(scylla_session)));
        let user_service = Arc::new(UserServiceImpl {
            repository: user_repo,
        });
        Container { user_service }
    }
}
