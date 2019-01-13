
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};





use crate::schema::todos;
use crate::schema::todos::dsl::*;

use crate::models::{Todo, NewTodo, Project};



pub struct TodoDB();

type DB = diesel::pg::Pg;



//impl TodoDB {
//
//    fn get_conn(&self) -> Result<PgPooledConnection, Error> {
//        self.0.get().map_err(|_| Error::ConnectionError)
//    }
//
//    pub fn list<T: Queryable<todos::SqlType, DB>>(&self) -> Result<Vec<T>, Error> {
//        todos.load::<T>(&self.get_conn()?).map_err(|_| Error::QueryError("Unable to get todos"))
//    }
//
//    pub fn set_in_progress(&self, todo: &Todo) -> Result<Todo, Error> {
//        
//        diesel::update(todos.filter(state.eq("in_progress")))
//            .set(state.eq("active"))
//            .execute(&self.get_conn()?)
//            .map_err(|_| Error::QueryError("Unable to unset in_progress todo"))?;
//
//        return diesel::update(todo)
//            .set(state.eq("in_progress"))
//            .get_result(&self.get_conn()?)
//            .map_err(|_| Error::QueryError("Unable to update todo"))
//    }
//
//    pub fn in_progress(&self) -> Result<Todo, Error> {
//        todos.filter(state.eq("in_progress"))
//            .first::<Todo>(&self.get_conn()?)
//            .map_err(|_| Error::QueryError("Unable to get todo"))
//    }
//
//    pub fn by_project(&self, project: &Project) -> Result<Vec<Todo>, Error> {
//        Todo::belonging_to(project)
//            .load::<Todo>(&self.get_conn()?)
//            .map_err(|_| Error::QueryError("Unable to get todos"))
//    }
//}