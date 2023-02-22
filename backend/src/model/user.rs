use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub name: String,
}
