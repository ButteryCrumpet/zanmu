extern crate actix;
extern crate actix_web;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use actix_web::{server, App, HttpRequest, HttpResponse, Error, Responder, http};

pub mod schema;
pub mod models;

use self::models::{Todo, NewTodo};

#[derive(Serialize)]
struct Thing {
    name: &'static str,
}

impl Responder for Thing {
    type Item = HttpResponse;
    type Error = Error;

    fn respond_to<S>(self, req: &HttpRequest<S>) -> Result<HttpResponse, Error> {
        let body = serde_json::to_string(&self)?;

        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body))
    }
}

fn index(req: &HttpRequest) -> impl Responder {
    Thing { name: "?" }
}

fn server() {
    server::new(
        || App::new()
            .resource("/", |r| r.f(index)))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run();
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL env variable must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn create_todo<'a>(conn: &PgConnection, title: &'a str, description: &'a str) -> Todo {
    use schema::todos;

    let new_todo = NewTodo {
        title: title,
        description: description,
        priority: false,
    };
    diesel::insert_into(todos::table)
        .values(&new_todo)
        .get_result(conn)
        .expect("Error saving")
}

pub fn get_todos(conn: &PgConnection) -> Vec<Todo> {
    use schema::todos::dsl::*;
    
    return todos.load::<Todo>(conn).unwrap();
}

fn main() {
    
    let connection = establish_connection();

    let inserted = create_todo(&connection, "First", "The first todo");
    println!("{}", inserted.title);
    let results = get_todos(&connection);
    println!("{}", "-------All Todos-------");
    for todo in results {
        println!("{}", todo.title);
    }
}