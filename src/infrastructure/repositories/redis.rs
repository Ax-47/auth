use crate::domain::repositories::redis as rd;
use async_trait::async_trait;
use redis::{AsyncCommands, Client, RedisResult};
use std::sync::Arc;
pub struct RedisRepository {
    pub redis_client: Arc<Client>,
}

impl RedisRepository {
    pub fn new(client: Arc<Client>) -> Self {
        RedisRepository {
            redis_client: client,
        }
    }
}
#[async_trait]
impl rd::RedisRepository for RedisRepository {
    async fn save_email(&self, comfirm_id: String, email: String) -> RedisResult<()> {
        let mut con = self.redis_client.get_multiplexed_tokio_connection().await?;
        let _ = con.set(comfirm_id, email).await?;
        Ok(())
    }
    async fn get_email(&self, comfirm_id: String) -> RedisResult<String> {
        let mut con = self.redis_client.get_multiplexed_tokio_connection().await?;
        let email = con.get(comfirm_id).await?;
        Ok(email)
    }
}
