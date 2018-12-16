#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;
pub mod db; 

use self::db::TodoDB;
use crate::models::{Todo, ListTodo};

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL env variable must be set");

    let db = TodoDB::new(&database_url).unwrap();

    let results = db.list::<ListTodo>().unwrap();
    for todo in results {
        println!("{}", serde_json::to_string(&todo).unwrap());
    }
}