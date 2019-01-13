use diesel::prelude::*;
use diesel::query_source::Table;
use diesel::backend::Backend;
use diesel::deserialize::Queryable;
use diesel::connection::Connection;
use diesel::query_builder::QueryFragment;
use diesel::insertable::CanInsertInSingleQuery;
use diesel::query_builder::AsChangeset;

//type ConnPool<T: Connection> = Pool<ConnectionManager<T>>;

#[derive(Debug)]
pub enum Error<'a> {
    ConnectionError,
    QueryError(&'a str)
}

type SqlType<T: Table> = <T::AllColumns as Expression>::SqlType;

// Dynamically sized, must box all queries?
pub trait Crud {
    type DB: Backend;
    type Tab: Table<FromClause = QueryFragment<Self::DB>>;
    type Conn: Connection<Backend = Self::DB>;

    fn get_conn(&self) -> &Self::Conn;

    fn get_table(&self) -> Self::Tab;

    fn create<T>(&self, new: T) -> Result<usize, Error> where
        T: Insertable<Self::Tab>,
        T::Values: QueryFragment<Self::DB> + CanInsertInSingleQuery<Self::DB>,
    {
        new.insert_into(self.get_table())
            .execute(self.get_conn())
            .map_err(|_| Error::QueryError("Unable to create todo"))
    }

    fn read<T>(&self, uid: i32) -> Result<T, Error> where
        T: Queryable<SqlType<Self::Tab>, Self::DB>
    {
        self.get_table()
            .find(uid)
            .load::<T>(self.get_conn())
            .map_err(|_| Error::QueryError("Unable to get todo"))
    }

    fn update<T>(&self, updatedItem: &T) -> Result<usize, Error> where
        T: AsChangeset<Target = Self::Tab>,
    {
        diesel::update(self.get_table())
            .set(updatedItem)
            .execute(self.get_conn())
            .map_err(|_| Error::QueryError("Unable to update todo"))
    }

    fn delete(&self, d_id: i32) -> Result<usize, Error> {
        diesel::delete(self.get_table().find(d_id))
            .execute(&self.get_conn())
            .map_err(|_| Error::QueryError("Unable to delete todo"))
    }

}