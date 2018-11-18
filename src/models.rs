use super::schema::todos;
use std::time::SystemTime;

#[derive(Queryable, Identifiable, AsChangeset)]
pub struct Todo {
    pub created: SystemTime,
    pub updated: SystemTime,
    pub id: i32,
    pub title: String,
    pub description: String,
    pub priority: bool,
    pub state: String 
}

#[derive(Insertable)]
#[table_name="todos"]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub description: &'a str,
    pub priority: bool,
}