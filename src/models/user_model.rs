use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user: String,
    pub password: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginInfo {
    pub user: String,
    pub password: String
}


#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize
}