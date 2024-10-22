use mongodb::bson::{self, oid::ObjectId};
use serde::{Deserialize, Serialize};

// User model (MongoDB document)
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub bio: Option<String>,
    pub contact: Option<String>,
    pub date_of_birth: Option<String>,
    pub gender: Option<String>,
    pub occupation: Option<String>,
    pub picture: Option<String>,
    pub verified: bool,
}

// User secret model for passwords
#[derive(Serialize, Deserialize, Debug)]
pub struct Secret {
    pub password: String,
    pub user_id: ObjectId,
}
