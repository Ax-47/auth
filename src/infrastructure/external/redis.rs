use redis::{Client, RedisResult};

pub fn fetch_an_integer() -> RedisResult<Client> {
    let uri = std::env::var("REDIS_URI").unwrap_or_else(|_| "redis://127.0.0.1/".to_string());
    let client = redis::Client::open(uri)?;
    Ok(client)
}
