use actix_web::{http::header, HttpResponse, ResponseError, body::MessageBody};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("The user name is already used.")]
    NameAlreadyTaken,

    #[error("Query error: {0}")]
    QueryError(#[from] sqlx::Error),
}

impl ResponseError for ServiceError {
    fn status_code(&self) -> reqwest::StatusCode {
        match self {
            ServiceError::NameAlreadyTaken => reqwest::StatusCode::BAD_REQUEST,
            ServiceError::QueryError(_) => reqwest::StatusCode::INTERNAL_SERVER_ERROR,
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
