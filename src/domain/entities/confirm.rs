use apalis::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Clone, Deserialize, Serialize)]
pub struct Confirm {
    pub id: Uuid,
    pub email: String,
}
impl Job for Confirm {
    const NAME: &'static str = "send_email";
}
