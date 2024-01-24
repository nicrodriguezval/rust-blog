use std::env;
use dotenv::dotenv;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use crate::schema::posts;
use crate::models::post::{Post, NewPost, SimplePost};

struct Conn {
    conn: PgConnection,
}

#[allow(dead_code)]
impl Conn {
    fn insert(&mut self, new_post: NewPost) {
        let post = diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result::<Post>(&mut self.conn)
            .expect("Error saving new post");

        println!("New post inserted: {:?}", post);
    }

    fn select_by_id(&mut self, id: i32) {
        let post = posts::table
            .find(id)
            .load::<Post>(&mut self.conn)
            .expect("Error loading posts");

        println!("Post selected: {:?}", post);
    }

    fn select_by_title(&mut self, title: &str) {
        let posts = posts::table
            .filter(posts::title.eq(title))
            .load::<Post>(&mut self.conn)
            .expect("Error loading posts");

        for post in &posts {
            println!("{:?}", post)
        }
    }

    fn select_all(&mut self) {
        let posts = posts::table
            .load::<Post>(&mut self.conn)
            .expect("Error loading posts");

        for post in &posts {
            println!("{:?}", post)
        }
    }

    fn select_simple(&mut self) {
        let posts = posts::table
            .select((posts::id, posts::title))
            .order(posts::id.desc())
            .limit(5)
            .load::<SimplePost>(&mut self.conn)
            .expect("Error loading posts");

        for post in &posts {
            println!("{:?}", post)
        }
    }

    fn update(&mut self, id: i32, title: &str, slug: &str) {
        let post = diesel::update(posts::table.find(id))
            .set((
                posts::title.eq(title),
                posts::slug.eq(slug),
            ))
            .get_result::<Post>(&mut self.conn)
            .expect("Error updating new post");

        println!("Post updated: {:?}", post);
    }

    fn delete(&mut self, title: &str) {
        let deleted = diesel::delete(posts::table.filter(posts::title.like(title)))
            .execute(&mut self.conn)
            .expect("Error deleting posts");

        println!("Deleted {} posts", deleted);
    }
}

#[allow(dead_code)]
pub fn run() {
    // Load .env file
    dotenv().ok();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let conn = PgConnection::establish(&db_url)
        .expect(&format!("Error connecting to {}", db_url));

    let mut conn = Conn { conn };

    let new_post = NewPost {
        title: "Hello, world 3!",
        slug: "hello-world-3",
        body: "This is my third post",
    };

    conn.insert(new_post);
    conn.select_all();
}

