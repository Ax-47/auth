use crate::domain::repositories::user::UserRepository;
use crate::domain::services::user::UserService;
use crate::infrastructure::external::scylla::connect_scylladb;
use crate::infrastructure::repositories::user_scylla::UserScyllaRepository;
use crate::infrastructure::services::user::UserServiceImpl;
use std::sync::Arc;
use tokio::runtime::Runtime;

pub struct Container {
    pub user_service: Arc<dyn UserService>,
}

impl Container {
    pub fn new() -> Self {
        let scylla_session = Runtime::new()
            .unwrap()
            .block_on(connect_scylladb())
            .unwrap();
        let user_repo: Arc<dyn UserRepository> =
            Arc::new(UserScyllaRepository::new(Arc::new(scylla_session)));
        let user_service = Arc::new(UserServiceImpl {
            repository: user_repo,
        });
        Container { user_service }
    }
}

impl Default for Container {
    fn default() -> Self {
        Self::new()
    }
}
