use crate::{models::user_model::{User, LoginInfo, LoginResponse, Claims}, middleware::authentication::AuthToken, repository::mongodb_repo::MongoRepo};
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::{results::InsertOneResult, bson::{oid::ObjectId, Document}};
use rocket::{http::Status, serde::json::Json, State};

#[post("/login", data = "<login_info>")]
pub fn login(db: &State<MongoRepo>,
    login_info: Json<LoginInfo>,
) -> Result<Json<LoginResponse>, Status> {
    let secret: &[u8] = b"worldrevolution1917"; // Replace with your actual secret key

    let username = &login_info.user.to_owned();
    let password = &login_info.password.to_owned();

    Claims {
        sub: username.clone(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize
    };

    
    let user_detail = db.login(username, password);

    match user_detail {
        Ok(claims) => {
            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret)).unwrap();
            Ok(Json(LoginResponse { token }))
        },
        Err(_) => Err(Status::InternalServerError),
    }

}

#[post("/user", data = "<new_user>")]
pub fn create_user(
    _auth: AuthToken, // Ensure this is working
    db: &State<MongoRepo>,
    new_user: Json<User>,
) -> Result<Json<InsertOneResult>, Status> {
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
}

#[get("/user/<path>")]
pub fn get_user(db: &State<MongoRepo>, path: String) -> Result<Json<Document>, Status> {
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
pub fn delete_user(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
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
pub fn get_all_users(db: &State<MongoRepo>) -> Result<Json<Vec<Document>>, Status> {
    let users = db.get_all_users();
    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}