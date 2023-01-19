use rocket::{get, http::Status};
use rocket::serde::json::Json;

use crate::database::config;
use crate::models::config::Config;
use crate::utils::database::Db;

#[get("/config")]
pub async fn get_config(
    db: Db
) -> Result<Json<Config>, Status> {

    db
        .run(|c| config::find_one(c, 1))
        .await
        .map(Json)
        .map_err(|_| Status::InternalServerError)
}