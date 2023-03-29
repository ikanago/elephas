use actix_web::{body::MessageBody, http::header, HttpResponse, ResponseError};
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("The user name is already used")]
    NameAlreadyTaken,
    #[error("The user is not found")]
    UserNotFound,
    #[error("Invalid ActivityPub request: {0}")]
    InvalidActivityPubRequest(String),
    #[error("User name or password is wrong")]
    WrongCredential,

    #[error("Internal server error")]
    InternalServerError,
    #[error("Query error: {0}")]
    QueryError(#[from] sqlx::Error),
    #[error("Key error: {0}")]
    KeyError(#[from] rsa::errors::Error),
    #[error("Failed to send request: {0}")]
    RequestError(#[from] reqwest::Error),
}

#[derive(ToSchema)]
pub struct ErrorMessage {
    pub error: String,
}

impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"{{"error": "{}"}}"#, self.error)
    }
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            ServiceError::NameAlreadyTaken
            | ServiceError::UserNotFound
            | ServiceError::InvalidActivityPubRequest(_) => reqwest::StatusCode::BAD_REQUEST,
            ServiceError::WrongCredential => reqwest::StatusCode::UNAUTHORIZED,
            ServiceError::InternalServerError
            | ServiceError::QueryError(_)
            | ServiceError::KeyError(_)
            | ServiceError::RequestError(_) => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let mut res = HttpResponse::new(self.status_code());
        res.headers_mut()
            .insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
        let message = ErrorMessage {
            error: self.to_string(),
        };
        let body = MessageBody::boxed(message.to_string());
        res.set_body(body)
    }
}
