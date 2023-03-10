use actix_web::{body::MessageBody, http::header, HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("The user name is already used.")]
    NameAlreadyTaken,
    #[error("Invalid ActivityPub request: {0}")]
    InvalidActivityPubRequest(String),
    #[error("Unauthorized")]
    Unauthorized,

    #[error("Internal server error")]
    InternalServerError,
    #[error("Query error: {0}")]
    QueryError(#[from] sqlx::Error),
    #[error("Key error: {0}")]
    KeyError(#[from] rsa::errors::Error),
    #[error("Failed to send request: {0}")]
    RequestError(#[from] reqwest::Error),
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            ServiceError::NameAlreadyTaken | ServiceError::InvalidActivityPubRequest(_) => {
                reqwest::StatusCode::BAD_REQUEST
            }
            ServiceError::Unauthorized => reqwest::StatusCode::UNAUTHORIZED,
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
        let body = MessageBody::boxed(format!("error: {}", self.to_string()));
        res.set_body(body)
    }
}
