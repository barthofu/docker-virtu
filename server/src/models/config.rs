use diesel::{Insertable, Queryable};
use serde::{Serialize, Deserialize};

use crate::schema::config;

#[derive(Serialize, Deserialize, Queryable, Debug, Insertable)]
#[table_name = "config"]
pub struct Config {
    id: i32,
    firstname: String,
    lastname: String,
}