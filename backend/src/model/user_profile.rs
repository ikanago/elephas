use serde::Serialize;
use utoipa::ToSchema;

use crate::model::{ap_person::ApPerson, user::User};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, ToSchema)]
pub struct UserProfile {
    #[schema(example = "alice")]
    pub name: String,
    #[schema(example = "Alice")]
    pub display_name: String,
    #[schema(example = "I am Alice.")]
    pub summary: String,
    #[schema(example = "https://example.com/avatar.png")]
    pub avatar_url: String,
}

impl From<User> for UserProfile {
    fn from(user: User) -> Self {
        Self {
            name: user.name,
            display_name: user.display_name,
            summary: user.summary,
            avatar_url: user.avatar_url,
        }
    }
}

impl From<ApPerson> for UserProfile {
    fn from(person: ApPerson) -> Self {
        Self {
            name: person.preferred_username,
            display_name: person.name.unwrap_or_default(),
            summary: person.summary.unwrap_or_default(),
            avatar_url: String::new(),
        }
    }
}
