mod error;
mod model;
pub mod routes;
mod service;

pub use error::ServiceError;

pub type Result<T> = std::result::Result<T, error::ServiceError>;

static SESSION_KEY: &str = "user_name";
