use crate::domain::error::ApiError;
use crate::domain::services::user::UserService;
use crate::{application::dtos::user, domain::error::ErrorMessage};
use actix_web::{web, Result};
use email_address::EmailAddress;

use uuid::Uuid;
pub async fn confirm_email(
    user_service: web::Data<dyn UserService>,
    post_data: web::Json<user::ConfirmEmailDTO>,
) -> Result<web::Json<user::ConfirmCreated>, ApiError> {
    let email = post_data.email.to_owned();
    if !EmailAddress::is_valid(email.as_str()) {
        return Err(ApiError::from(ErrorMessage {
            message: "this is not email".to_owned(),
            code: 400,
        }));
    }
    let is_email_exist = user_service.is_email_exist(email.clone()).await?;

    if is_email_exist {
        return Err(ApiError::from(ErrorMessage {
            message: "this email is exist on database".to_owned(),
            code: 400,
        }));
    }
    let uuid_gen = Uuid::new_v4();
    user_service.create_confirm(uuid_gen, email.clone()).await?;
    // TODO add to redis
    Ok(web::Json(user::ConfirmCreated {
        message: "send".to_string(),
    }))
}
