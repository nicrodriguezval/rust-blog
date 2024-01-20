use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    let db_url = env::var("DB_URL")
        .expect("DATABASE_URL must be set");

    println!("DB_URL: {:?}", db_url);
}
