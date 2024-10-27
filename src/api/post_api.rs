

use crate::{models::post_model::Post, repository::mongodb_repo::MongoRepo};
use mongodb::{results::InsertOneResult, bson::{oid::ObjectId, Document}};
use rocket::{http::Status, serde::json::Json, State};

#[post("/post", data = "<new_post>")]
pub fn create_post(
    db: &State<MongoRepo>,
    new_post: Json<Post>,
) -> Result<Json<InsertOneResult>, Status> {
    let data: Post = Post {
        id: None,
        name: new_post.name.to_owned(),
        date: new_post.date.to_owned(),
        text: new_post.text.to_owned(),
        description: new_post.description.to_owned(),
        author: new_post.author,
    };
    let post_detail = db.create_post(data);
    match post_detail {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/post/<path>")]
pub fn get_post(db: &State<MongoRepo>, path: String) -> Result<Json<Document>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let post_detail = db.get_post(&id);
    match post_detail {
        Ok(post) => Ok(Json(post)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/post/<path>", data = "<new_post>")]
pub fn update_post(
    db: &State<MongoRepo>,
    path: String,
    new_post: Json<Post>,
) -> Result<Json<Document>, Status> {
    if path.is_empty() {
        return Err(Status::BadRequest);
    }

    let post_id = ObjectId::parse_str(&path).map_err(|_| Status::BadRequest)?;
    let data = Post {
        id: Some(post_id),
        name: new_post.name.to_owned(),
        date: new_post.date.to_owned(),
        text: new_post.text.to_owned(),
        description: new_post.description.to_owned(),
        author: new_post.author,
    };

    match db.update_post(&path, data) {
        Ok(update) => {
            if update.matched_count == 1 {
                match db.get_post(&path) {
                    Ok(post) => Ok(Json(post)),
                    Err(_) => Err(Status::InternalServerError),
                }
            } else {
                Err(Status::NotFound)
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/post/<path>")]
pub fn delete_post(db: &State<MongoRepo>, path: String) -> Result<Json<&str>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = db.delete_post(&id);
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return Ok(Json("Post successfully deleted!"));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/posts")]
pub fn get_all_posts(db: &State<MongoRepo>) -> Result<Json<Vec<Document>>, Status> {
    let posts = db.get_all_posts();
    match posts {
        Ok(posts) => Ok(Json(posts)),
        Err(_) => Err(Status::InternalServerError),
    }
}