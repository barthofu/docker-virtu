use rocket_sync_db_pools::database;

#[database("postgres")]
pub struct Db(diesel::PgConnection);