extern crate diesel;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use schema::todos;
use schema::todos::dsl::*;

struct TodoDB(PgConnection);

impl TodosDB {

    fn create(&self, new_todo: &NewTodo) -> QueryResult<Todo> {

        diesel::insert_into(todos::table)
            .values(new_todo)
            .get_result(self)
    };

    fn update(&self, updated_todo: &Todo) -> QueryResult<Todo> {

        diesel::update(todos::table)
            .set(updated_todo)
            .get_result(self)
    };

    fn set_in_progress(&self, todo: &Todo) -> QueryResult<Todo> {
        
        diesel::update(posts.filter(state.eq("in_progress")))
            .set("active")
            .execute(self)?;

        diesel::update(todo)
            .set(state.eq("in_progress"))
            .get_result(self)?;
    };

    fn get(&self, id: i32) -> QueryResult<Todo> {
        todos::table.find(id).first::<Todo>(self)
    };

    fn in_progress(&self) -> QueryResult<Todo> {
        posts.filter(state.eq("in_progress"))
            .limit(1)
            .load<Todo>(&self);
    };

    fn all(&self) -> QueryResult<Vec<Todo>> {
        todos.load::<Todo>(conn);
    };

    fn by_project(&self, project: &Project) -> Queryable<Vec<Todo>> {
        Todo::belogining_to(project)
            .load::<Todo>(self);
    };
}