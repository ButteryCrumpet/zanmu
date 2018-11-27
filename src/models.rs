use super::schema::{projects, todos};
use std::time::SystemTime;

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