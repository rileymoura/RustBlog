use crate::{models::user_model::{User, LoginInfo, LoginResponse, Claims}, middleware::authentication::AuthToken, repository::mongodb_repo::MongoRepo};
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::{results::InsertOneResult, bson::{oid::ObjectId, Document}};
use rocket::{http::Status, serde::json::Json, State};

#[post("/login", data = "<login_info>")]
pub fn login(
    db: &State<MongoRepo>,
    login_info: Json<LoginInfo>,
) -> Result<Json<LoginResponse>, Status> {
    let secret: &[u8] = b"worldrevolution1917"; // Ensure this is secure and not hard-coded in production

    let username = &login_info.username;
    let password = &login_info.password;

    // Create claims after validating the user
    let user_detail = db.login(username, password);

    println!("Login attempt for user {} with password {}", username, password);

    match user_detail {
        Ok(_claims) => {
            let claims = Claims {
                sub: username.to_owned(), // Ensure this is set correctly
                exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
            };

            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret)).map_err(|_| Status::InternalServerError)?;
            println!("Generated token: {}", token);
            Ok(Json(LoginResponse { token }))
        },
        Err(e) => {
            println!("Login failed for user {}: {}", username, e); // log more details for debugging
            Err(Status::Unauthorized)
        },
    }
}


#[post("/user", data = "<new_user>")]
pub fn create_user(
    _auth: AuthToken, // Ensure this is working
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {
    match db.find_user_by_username(&new_user.user) {
        Ok(Some(_)) => Err(Status::Conflict), // change for json response l8r
        Ok(None) => {
            let data = User {
                id: None,
                name: new_user.name.to_owned(),
                user: new_user.user.to_owned(),
                password: new_user.password.to_owned(),
            };
        
            // Call the synchronous method
            match db.create_user(data) {
                Ok(result) => Ok(Json(result)), // Ensure result is of type InsertOneResult
                Err(_) => Err(Status::InternalServerError),
            }       
        },
        Err(_) => Err(Status::InternalServerError)
    }
}

#[get("/user/<path>")]
pub fn get_user(_auth: AuthToken, db: &State<MongoRepo>, path: String) -> Result<Json<Document>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let user_detail = db.get_user(&id);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/user/<path>", data = "<new_user>")]
pub fn update_user(
    _auth: AuthToken, //
    db: &State<MongoRepo>,
    path: String,
    new_user: Json<User>,
) -> Result<Json<Document>, Status> {
    if path.is_empty() {
        return Err(Status::BadRequest);
    }

    let user_id = ObjectId::parse_str(&path).map_err(|_| Status::BadRequest)?;
    let data = User {
        id: Some(user_id),
        name: new_user.name.to_owned(),
        user: new_user.user.to_owned(),
        password: new_user.password.to_owned()
    };

    match db.update_user(&path, data) {
        Ok(update) => {
            if update.matched_count == 1 {
                match db.get_user(&path) {
                    Ok(user) => Ok(Json(user)),
                    Err(_) => Err(Status::InternalServerError),
                }
            } else {
                Err(Status::NotFound)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/user/<path>")]
pub fn delete_user(_auth: AuthToken, db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_user(&id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("User successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/users")]
pub fn get_all_users(_auth: AuthToken, db: &State<MongoRepo>) -> Result<Json<Vec<Document>>, Status> {
    let users = db.get_all_users();
    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}