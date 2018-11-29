extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

use schema::todos;
use schema::todos::dsl::*;

use models::{Todo, NewTodo, Project};

type PgPool = Pool<ConnectionManager<PgConnection>>;
type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub struct TodoDB(PgPool);

type DB = diesel::pg::Pg;

#[derive(Debug)]
pub enum Error<'a> {
    ConnectionError,
    QueryError(&'a str)
}


impl TodoDB {

    pub fn new(uri: &str) -> Result<TodoDB, PoolError> {
        let manager = ConnectionManager::<PgConnection>::new(uri);
        Ok(TodoDB(Pool::builder().build(manager)?))
    }

    fn get_conn(&self) -> Result<PgPooledConnection, Error> {
        self.0.get().map_err(|_| Error::ConnectionError)
    }

    //pub fn create<T: Insertable<todos::SqlType>>(&self, new_todo: &T) -> Result<Todo, Error> {
//
    //    diesel::insert_into(todos::table)
    //        .values(new_todo)
    //        .get_result(&self.get_conn()?)
    //        .map_err(|_| Error::QueryError("Unable to create todo"))
    //}

     pub fn read<T: Queryable<todos::SqlType, DB>>(&self, todo_id: i32) -> Result<T, Error> {
        todos::table.find(todo_id)
            .first::<T>(&self.get_conn()?)
            .map_err(|_| Error::QueryError("Unable to get todo"))
    }

    pub fn update(&self, updated_todo: &Todo) -> Result<Todo, Error> {

        diesel::update(todos::table)
            .set(updated_todo)
            .get_result(&self.get_conn()?)
            .map_err(|_| Error::QueryError("Unable to update todo"))
    }

    pub fn delete(&self, d_id: i32) -> Result<usize, Error> {
        diesel::delete(todos.filter(id.eq(d_id)))
            .execute(&self.get_conn()?)
            .map_err(|_| Error::QueryError("Unable to delete todo"))
    }

    pub fn list<T: Queryable<todos::SqlType, DB>>(&self) -> Result<Vec<T>, Error> {
        todos.load::<T>(&self.get_conn()?).map_err(|_| Error::QueryError("Unable to get todos"))
    }

    pub fn set_in_progress(&self, todo: &Todo) -> Result<Todo, Error> {
        
        diesel::update(todos.filter(state.eq("in_progress")))
            .set(state.eq("active"))
            .execute(&self.get_conn()?)
            .map_err(|_| Error::QueryError("Unable to unset in_progress todo"))?;

        return diesel::update(todo)
            .set(state.eq("in_progress"))
            .get_result(&self.get_conn()?)
            .map_err(|_| Error::QueryError("Unable to update todo"))
    }

    pub fn in_progress(&self) -> Result<Todo, Error> {
        todos.filter(state.eq("in_progress"))
            .first::<Todo>(&self.get_conn()?)
            .map_err(|_| Error::QueryError("Unable to get todo"))
    }

    pub fn by_project(&self, project: &Project) -> Result<Vec<Todo>, Error> {
        Todo::belonging_to(project)
            .load::<Todo>(&self.get_conn()?)
            .map_err(|_| Error::QueryError("Unable to get todos"))
    }
}