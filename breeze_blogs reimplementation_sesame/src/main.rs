#[macro_use] extern crate rocket;

mod db;
mod routes;   // 👈 this line is critical

use rocket::http::Status;
use sesame_rock::prelude::*;
use sesame_rock::policy::NoPolicy;

#[launch]
fn rocket() -> _ {
    match db::establish_connection() {
        Ok(_) => println!("🚀 Database connection test passed!"),
        Err(e) => eprintln!("❌ Database connection failed: {:?}", e),
    }
    rocket::build()
        .attach(PConFairing)     // required for Sesame
        .manage(YouContext::new())
        .mount("/", routes![
            routes::register
        ])
}
