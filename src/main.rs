mod api; 
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::user_api::{create_user, get_user, update_user, delete_user, get_all_users};
use api::post_api::{create_post, get_post, update_post, delete_post, get_all_posts};
use repository::mongodb_repo::MongoRepo;

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build()
        .manage(db)

        // USERS
        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
        .mount("/", routes![update_user])
        .mount("/", routes![delete_user])
        .mount("/", routes![get_all_users])

        // POSTS
        .mount("/", routes![create_post])
        .mount("/", routes![get_post])
        .mount("/", routes![update_post])
        .mount("/", routes![delete_post])
        .mount("/", routes![get_all_posts])
}