use rocket::{routes, launch, catchers};

use server::routes::{self, errors};
use server::middlewares::cors::Cors;
use server::utils::database::Db;

#[launch]
fn rocket() -> _ {

    println!("Starting server...");
    
    rocket::build()
        .attach(Cors)
        .attach(Db::fairing())
        .register("/", catchers![errors::not_found, errors::default, errors::internal_error])
        .mount("/", routes![

            routes::index::index,
            routes::random::get_random,
            routes::config::get_config,
            
        ])
}