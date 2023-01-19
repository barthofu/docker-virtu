use diesel::prelude::*;

use crate::models::config::Config;

pub fn find_one(conn: &PgConnection, id: i32) -> QueryResult<Config> {
    
    crate::schema::config::table.find(id).first::<Config>(conn)
}