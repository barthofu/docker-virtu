use rocket::{get, http::Status};
use rand::Rng;
use rocket::serde::json::Json;

#[get("/random")]
pub async fn get_random() -> Result<Json<i32>, Status> {

    let mut rng = rand::thread_rng();

    Ok(Json(rng.gen_range(0..100)))
}