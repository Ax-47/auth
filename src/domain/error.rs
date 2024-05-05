use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorMessage {
    pub message: String,
    pub code: u32,
}

impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}, Code: {}", self.message, self.code)
    }
}

#[derive(Debug)]
pub struct ApiError(ErrorMessage);

impl From<ErrorMessage> for ApiError {
    fn from(error: ErrorMessage) -> ApiError {
        ApiError(error)
    }
}
impl From<scylla::transport::errors::QueryError> for ApiError {
    fn from(e: scylla::transport::errors::QueryError) -> ApiError {
        println!("{}", e);
        ApiError(ErrorMessage {
            message: "database broke".to_owned(),
            code: 500,
        })
    }
}
impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl actix_web::ResponseError for ApiError {
    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::BadRequest().json(&self.0)
    }
}

#[derive(Debug)]
pub struct RepositoryError {
    pub message: String,
}

impl Into<ErrorMessage> for RepositoryError {
    fn into(self) -> ErrorMessage {
        ErrorMessage {
            message: self.message,
            code: 1,
        }
    }
}
