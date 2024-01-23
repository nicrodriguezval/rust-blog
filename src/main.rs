#[macro_use]
extern crate diesel;

mod schema;
mod models;

use std::env;
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use schema::posts::dsl::posts;
use models::post::Post;

fn main() {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let mut conn = PgConnection::establish(&db_url)
        .expect(&format!("Error connecting to {}", db_url));

    // SELECT * FROM posts;
    let posts_result = posts
        .load::<Post>(&mut conn)
        .expect("Error loading posts");

    for post in &posts_result {
        println!("{}: {}", post.id, post.title);
    }
}
