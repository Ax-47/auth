use crate::domain::entities::confirm;
use anyhow;
use apalis::{redis, redis::RedisStorage};
pub async fn init_apalis() -> anyhow::Result<RedisStorage<confirm::Confirm>> {
    let uri = std::env::var("REDIS_URI").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let redis_conn = redis::connect(uri).await?;
    let storage = RedisStorage::new(redis_conn);
    Ok(storage)
}
