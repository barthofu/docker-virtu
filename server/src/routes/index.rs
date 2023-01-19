use rocket::{get};
use rocket::serde::json::Json;

#[get("/")]
pub async fn index() -> Json<String> {

    Json("API is running!".to_owned())
}