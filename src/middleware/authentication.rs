use jsonwebtoken::{decode, DecodingKey, Validation, TokenData};
use rocket::http::Status;
use rocket::async_trait;  // Ensure this is included if you're using async
use rocket::request::{FromRequest, Outcome, Request};
//use rocket::route::Outcome;
use rocket::serde::json::Json;
use serde_json::json;
use crate::models::user_model::Claims;

fn verify_token(token: &str, secret: &[u8]) -> Result<TokenData<Claims>, Status> {
    let validation = Validation::default();
    
    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(secret), &validation)
        .map_err(|err| {
            eprintln!("Token verification error: {:?}", err);
            Status::Unauthorized
        })?;

    println!("Verified claims: {:?}", token_data.claims); // For debugging
    Ok(token_data)
}

pub struct AuthToken;

#[async_trait]
impl<'r> FromRequest<'r> for AuthToken {
    type Error = Json<serde_json::Value>;
    
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let secret: &[u8] = b"worldrevolution1917"; // Replace with your actual secret key

        if let Some(token) = request.headers().get_one("Authorization") {
            if let Some(token) = token.strip_prefix("Bearer ") {
                match verify_token(token, secret) {
                    Ok(token_data) => {
                        let _claims = token_data.claims;
                        Outcome::Success(AuthToken)
                    },
                    Err(_) => {
                        Outcome::Error((Status::Unauthorized, Json(json!( {
                            "error": "Unauthorized",
                            "message": "Invalid token."
                        }))))
                    },
                }
            } else {
                Outcome::Error((Status::BadRequest, Json(json!( {
                    "error": "Bad Request",
                    "message": "Invalid token format."
                }))))
            }
        } else {
            Outcome::Error((Status::Unauthorized, Json(json!( {
                "error": "Unauthorized",
                "message": "No token provided."
            }))))
        }
    }

    
}
