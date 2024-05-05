use crate::domain::error::ApiError;
use crate::domain::services::user::UserService;
use crate::{application::dtos::user, domain::error::ErrorMessage};
use actix_web::{web, HttpResponse, Result};
use email_address::EmailAddress;
pub async fn confirm_email(
    user_service: web::Data<dyn UserService>,
    post_data: web::Json<user::ConfirmEmailDTO>,
) -> Result<(), ApiError> {
    let email = post_data.email.to_owned();
    if !EmailAddress::is_valid(email.as_str()) {
        return Err(ApiError::from(ErrorMessage {
            message: "this is not email".to_owned(),
            code: 400,
        }));
    }
    let is_email_exist = user_service.is_email_exist(email).await?;
    if is_email_exist {
        return Err(ApiError::from(ErrorMessage {
            message: "this email is exist on database".to_owned(),
            code: 400,
        }));
    }

    Ok(())
}
