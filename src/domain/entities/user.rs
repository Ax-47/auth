use scylla::SerializeRow;
use serde::Deserialize;
use uuid::Uuid;
#[derive(Clone, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub tag: String,
    pub email: String,
    pub password: String,
    pub role: String,
}
#[derive(Clone, Deserialize, SerializeRow)]
pub struct CreateUser {
    pub name: String,
    pub tag: String,
    pub email: String,
    pub password: String,
    pub role: String,
}
