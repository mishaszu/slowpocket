use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub hash: String,
    #[serde(skip_serializing)]
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub password: Option<PasswordUpdate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordUpdate {
    pub old_password: String,
    pub new_password: String,
}
