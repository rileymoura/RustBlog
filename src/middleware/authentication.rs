use jsonwebtoken::{decode, DecodingKey, Validation, TokenData};
use rocket::http::Status;
use rocket::async_trait;  // Ensure this is included if you're using async
use rocket::request::{FromRequest, Request};
use rocket::serde::json::Json;
use serde_json::json;
use crate::models::user_model::Claims;

pub fn verify_token(token: &str, secret: &[u8]) -> Result<TokenData<Claims>, Status> {
    let validation = Validation::default();
    decode::<Claims>(token, &DecodingKey::from_secret(secret), &validation)
        .map_err(|_| Status::Unauthorized)
}

#[derive(Debug)]
pub struct AuthToken(pub Claims);

#[async_trait]
impl<'r> FromRequest<'r> for AuthToken {
    type Error = Json<serde_json::Value>;

    async fn from_request(request: &'r Request<'_>) -> Result<Self, Self::Error> {
        let secret: &[u8] = b"your_secret_key_here"; // Replace with your actual secret key

        if let Some(token) = request.headers().get_one("Authorization") {
            if let Some(token) = token.strip_prefix("Bearer ") {
                match verify_token(token, secret) {
                    Ok(claims) => Ok(AuthToken(claims.claims)),
                    Err(_) => Err(Json(json!( {
                        "error": "Unauthorized",
                        "message": "Invalid token."
                    }))),
                }
            } else {
                Err(Json(json!( {
                    "error": "Bad Request",
                    "message": "Invalid token format."
                })))
            }
        } else {
            Err(Json(json!( {
                "error": "Unauthorized",
                "message": "No token provided."
            })))
        }
    }
}
