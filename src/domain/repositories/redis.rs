use async_trait::async_trait;
use redis::RedisResult;

#[async_trait]
pub trait RedisRepository: Send + Sync {
    async fn save_email(&self, comfirm_id: String, email: String) -> RedisResult<()>;
    async fn get_email(&self, comfirm_id: String) -> RedisResult<String>;
}
