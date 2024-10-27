use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
    bson::{doc, oid::ObjectId, to_document, Document}, // Import Document here
    results::{InsertOneResult, UpdateResult, DeleteResult},
    sync::{Client, Collection},
    error::Error as MongoError,
};

use crate::models::user_model::User;
use crate::models::post_model::Post;

pub struct MongoRepo {
    user_col: Collection<Document>, // Use Document here
    post_col: Collection<Document>, // Use Document here
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = env::var("MONGOURI").unwrap_or_else(|_| "Error loading env variable".to_string());
        let client = Client::with_uri_str(&uri).expect("Failed to initialize client");
        let db = client.database("RustBlog");

        let user_col: Collection<Document> = db.collection("User");
        let post_col: Collection<Document> = db.collection("Post");

        MongoRepo { user_col, post_col }
    }

    pub fn login(&self, username: &str, password: &str) -> Result<Document, MongoError> {
        let filter = doc! {"user": username, "password": password};
        let user_detail = self.user_col.find_one(filter, None)?
            .ok_or_else(|| MongoError::custom("User not found"))?;
        Ok(user_detail)
    }

    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, MongoError> {
        let user_doc = to_document(&new_user).map_err(MongoError::from)?; // Convert User to Document
        self.user_col.insert_one(user_doc, None)
    }

    pub fn get_user(&self, id: &str) -> Result<Document, MongoError> {
        let obj_id = ObjectId::parse_str(id).map_err(|e| MongoError::custom(format!("Invalid ObjectId: {}", e)))?;
        let filter = doc! {"_id": obj_id};
        let user_detail = self.user_col.find_one(filter, None)?
            .ok_or_else(|| MongoError::custom("User not found"))?;
        Ok(user_detail)
    }

    pub fn update_user(&self, id: &str, new_user: User) -> Result<UpdateResult, MongoError> {
        let obj_id = ObjectId::parse_str(id).map_err(|e| MongoError::custom(format!("Invalid ObjectId: {}", e)))?;

        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set": {
                "id": new_user.id,
                "name": new_user.name,
            },
        };
        self.user_col.update_one(filter, new_doc, None)
    }

    pub fn delete_user(&self, id: &str) -> Result<DeleteResult, MongoError> {
        let obj_id = ObjectId::parse_str(id).map_err(|e| MongoError::custom(format!("Invalid ObjectId: {}", e)))?;
        let filter = doc! {"_id": obj_id};
        let result = self.user_col.delete_one(filter, None)?;
        Ok(result)
    }

    pub fn get_all_users(&self) -> Result<Vec<Document>, MongoError> {
        let cursors = self.user_col.find(None, None)?;
        let users: Result<Vec<_>, _> = cursors.collect();
        users.map_err(Into::into)
    }

    pub fn create_post(&self, new_post: Post) -> Result<InsertOneResult, MongoError> {
        let post_doc = to_document(&new_post).map_err(MongoError::from)?; // Convert Post to Document
        self.post_col.insert_one(post_doc, None)
    }

    pub fn get_post(&self, id: &str) -> Result<Document, MongoError> {
        let obj_id = ObjectId::parse_str(id).map_err(|e| MongoError::custom(format!("Invalid ObjectId: {}", e)))?;
        let filter = doc! {"_id": obj_id};
        let post_detail = self.post_col.find_one(filter, None)?
            .ok_or_else(|| MongoError::custom("Post not found"))?;
        Ok(post_detail)
    }

    pub fn update_post(&self, id: &str, new_post: Post) -> Result<UpdateResult, MongoError> {
        let obj_id = ObjectId::parse_str(id).map_err(|e| MongoError::custom(format!("Invalid ObjectId: {}", e)))?;

        let filter = doc! {"_id": obj_id};
        let update_doc = doc! {
            "$set": {
                "name": new_post.name,
                "date": new_post.date,
                "text": new_post.text,
                "description": new_post.description,
                "author": new_post.author,
            },
        };
        self.post_col.update_one(filter, update_doc, None)
    }

    pub fn delete_post(&self, id: &str) -> Result<DeleteResult, MongoError> {
        let obj_id = ObjectId::parse_str(id).map_err(|e| MongoError::custom(format!("Invalid ObjectId: {}", e)))?;
        let filter = doc! {"_id": obj_id};
        self.post_col.delete_one(filter, None)
    }

    pub fn get_all_posts(&self) -> Result<Vec<Document>, MongoError> {
        let cursors = self.post_col.find(None, None)?;
        let posts: Result<Vec<_>, _> = cursors.collect();
        posts.map_err(Into::into)
    }
}
