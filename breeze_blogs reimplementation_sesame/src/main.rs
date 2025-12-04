#[macro_use]
extern crate rocket;

mod policy;
mod routes;

use sesame_rocket::rocket::{SesameRocket, routes};

#[rocket::main]
async fn main() {
    if let Err(e) = SesameRocket::build()
        .mount(
            "/",
            routes![
                routes::register,
            ],
        )
        .launch()
        .await
    {
        eprintln!("❌ Rocket failed to launch: {:?}", e);
    }
}
