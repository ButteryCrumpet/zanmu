use std::time::SystemTime;
use diesel::prelude::*;

use super::schema::{projects, todos};

type DB = diesel::pg::Pg;

#[derive(Queryable, Identifiable, AsChangeset, Associations, Debug)]
#[belongs_to(Project)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub priority: bool,
    pub state: String,
    pub created: SystemTime,
    pub updated: SystemTime,
    pub project_id: Option<i32>, 
}

#[derive(Debug)]
pub struct ListTodo {
    pub id: i32,
    pub title: String,
    pub state: String,
}

impl Queryable<todos::SqlType, DB> for ListTodo {
    type Row = (i32, String, String, bool, String, SystemTime, SystemTime, Option<i32>);

    fn build(row: Self::Row) -> Self {
        ListTodo {
            id: row.0,
            title: row.1,
            state: row.4,
        }
    }
}

#[derive(Insertable)]
#[table_name="todos"]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub project_id: Option<i32>,
    pub description: &'a str,
    pub priority: bool,
}

#[derive(Queryable, Identifiable, AsChangeset, Debug)]
pub struct Project {
    pub id: i32,
    pub title: String,
    pub description: String
}

#[derive(Insertable)]
#[table_name="projects"]
pub struct NewProject<'a> {
    pub title: &'a str,
    pub description: &'a str
}