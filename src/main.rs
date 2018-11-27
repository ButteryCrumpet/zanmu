#[macro_use]
extern crate diesel;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;
pub mod db; 

use self::db::TodoDB;
  
fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL env variable must be set");

    let db = TodoDB::new(&database_url).unwrap();

    let results = db.list().unwrap();
    for todo in results {
        println!("{:?}", todo);
    }
}