mod error;
mod model;
pub mod routes;

pub type Result<T> = std::result::Result<T, error::ServiceError>;
