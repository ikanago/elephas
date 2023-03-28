mod error;
mod model;
pub mod routes;

pub type Result<T> = std::result::Result<T, error::ServiceError>;

static SESSION_KEY: &str = "user_name";
