use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub enum Role {
    Admin,
    User,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserMongoDb {
    pub _id: ObjectId,
    pub email: String,
    pub password_hash: String,
    pub role: Role,
    pub is_activated: bool,
    pub created_at: DateTime,
    pub modified_at: DateTime,
}

pub struct User {
    pub id: String,
    pub email: String,
    pub role: Role,
    pub is_activated: bool,
    pub created_at: DateTime,
    pub modified_at: DateTime,
}

impl Into<UserCreate> for UserMongoDb {
    fn into(self) -> UserCreate {
        UserCreate {
            email: self.email,
            password_hash: self.password_hash,
        }
    }
}

impl Into<User> for UserMongoDb {
    fn into(self) -> User {
        User {
            id: self._id.to_hex(),
            email: self.email,
            role: self.role,
            is_activated: self.is_activated,
            created_at: self.created_at,
            modified_at: self.modified_at,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserCreate {
    pub email: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserPatch {
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub role: Option<Role>,
    pub is_activated: Option<bool>,
}
