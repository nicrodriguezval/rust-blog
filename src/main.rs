#[macro_use]
extern crate diesel;

mod orm_examples;
mod schema;
mod models;

fn main() {
    orm_examples::run();
}