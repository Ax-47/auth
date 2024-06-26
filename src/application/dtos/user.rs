use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ConfirmEmailDTO {
    pub email: String,
}
#[derive(Deserialize, Serialize)]
pub struct ConfirmCreated {
    pub message: String,
}
