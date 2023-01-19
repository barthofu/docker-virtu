use rocket::{routes, launch, catchers};

use server::routes::{self, errors};
use server::middlewares::cors::Cors;

#[launch]
fn rocket() -> _ {

    println!("Starting server...");
    
    rocket::build()
        .attach(Cors)
        .register("/", catchers![errors::not_found, errors::default, errors::internal_error])
        .mount("/", routes![

            routes::index::index,
            routes::random::get_random,
            
        ])
}