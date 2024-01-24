#[macro_use]
extern crate diesel;

mod schema;
mod models;

use std::env;
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use models::post::{Post};
use schema::posts;

fn main() {
    // Load .env file
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let mut conn = PgConnection::establish(&db_url)
        .expect(&format!("Error connecting to {}", db_url));

    // INSERT INTO posts (title, slug, body) VALUES ($1, $2, $3) RETURNING id, title, slug, body;
    // let new_post = NewPost {
    //     title: "Hello, world!",
    //     slug: "hello-world",
    //     body: "This is my first post",
    // };

    // diesel::insert_into(posts::table)
    //     .values(&new_post)
    //     .get_result::<Post>(&mut conn)
    //     .expect("Error saving new post");

    // SELECT * FROM posts WHERE id = 5 AND title = 'Hello, world!';
    // let posts_result = posts::table
    //     .find(5)
    //     .filter(posts::title.eq("Hello, world!"))
    //     .load::<Post>(&mut conn)
    //     .expect("Error loading posts");

    // for post in posts_result.into_iter() {
    //     println!("{:?}", post)
    // }

    // SELECT id, title FROM posts ORDER BY id DESC LIMIT 5;
    // let posts_result = posts::table
    //     .select((posts::id, posts::title))
    //     .order(posts::id.desc())
    //     .limit(5)
    //     .load::<SimplePost>(&mut conn)
    //     .expect("Error loading posts");

    // for post in &posts_result {
    //     println!("{:?}", post)
    // }

    // UPDATE posts SET title = $1, slug = $2 WHERE id = $3 RETURNING id, title, slug, body;
    diesel::update(posts::table.find(5))
        .set((
            posts::title.eq("Hello, world. Updated!"),
            posts::slug.eq("hello-world-updated"),
        ))
        .get_result::<Post>(&mut conn)
        .expect("Error saving new post");

    // SELECT * FROM posts;
    let posts_result = posts::table
        .load::<Post>(&mut conn)
        .expect("Error loading posts");

    for post in posts_result.into_iter() {
        println!("{:?}", post)
    }
}
