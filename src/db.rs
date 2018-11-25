extern crate diesel;

use diesel::prelude::*;
use diesel::result::ConnectionError;
use diesel::pg::PgConnection;
use schema::todos;
use schema::todos::dsl::*;

use models::{Todo, NewTodo, Project};

pub struct TodoDB {
    conn: PgConnection
}

impl TodoDB {

    pub fn new(uri: &str) -> Result<TodoDB, ConnectionError> {
        let conn = PgConnection::establish(&uri)?;
        Ok(TodoDB{conn: conn})
    }

    pub fn create(&self, new_todo: &NewTodo) -> QueryResult<Todo> {

        diesel::insert_into(todos::table)
            .values(new_todo)
            .get_result(&self.conn)
    }

    pub fn update(&self, updated_todo: &Todo) -> QueryResult<Todo> {

        diesel::update(todos::table)
            .set(updated_todo)
            .get_result(&self.conn)
    }

    pub fn set_in_progress(&self, todo: &Todo) -> QueryResult<Todo> {
        
        diesel::update(todos.filter(state.eq("in_progress")))
            .set(state.eq("active"))
            .execute(&self.conn)?;

        return diesel::update(todo)
            .set(state.eq("in_progress"))
            .get_result(&self.conn);
    }

    pub fn get(&self, todo_id: i32) -> QueryResult<Todo> {
        todos::table.find(todo_id).first::<Todo>(&self.conn)
    }

    pub fn in_progress(&self) -> QueryResult<Todo> {
        todos.filter(state.eq("in_progress"))
            .limit(1)
            .get_result(&self.conn)
    }

    pub fn all(&self) -> QueryResult<Vec<Todo>> {
        todos.load::<Todo>(&self.conn)
    }

    pub fn by_project(&self, project: &Project) -> QueryResult<Vec<Todo>> {
        Todo::belonging_to(project)
            .load::<Todo>(&self.conn)
    }
}