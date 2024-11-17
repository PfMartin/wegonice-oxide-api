use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Role {
    Admin,
    User,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserDb {
    pub id: Option<ObjectId>,
    pub email: String,
    pub password_hash: String,
    pub role: Role,
    pub is_activated: bool,
    pub created_at: bson::DateTime,
    pub modified_at: bson::DateTime,
}

pub struct User {
    pub id: String,
    pub email: String,
    pub role: Role,
    pub is_activated: bool,
    pub created_at: bson::DateTime,
    pub modified_at: bson::DateTime,
}

impl Into<User> for UserDb {
    fn into(self) -> User {
        let id = match self.id {
            Some(value) => value.to_hex(),
            None => String::from(""),
        };

        User {
            id,
            email: self.email,
            role: self.role,
            is_activated: self.is_activated,
            created_at: self.created_at,
            modified_at: self.modified_at,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserCreate {
    pub email: String,
    pub password_hash: String,
}
