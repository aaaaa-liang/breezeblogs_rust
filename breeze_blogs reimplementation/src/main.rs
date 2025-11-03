#[macro_use] extern crate rocket;

mod db;
mod routes;   // 👈 this line is critical

#[launch]
fn rocket() -> _ {
    match db::establish_connection() {
        Ok(_) => println!("🚀 Database connection test passed!"),
        Err(e) => eprintln!("❌ Database connection failed: {:?}", e),
    }

    rocket::build()
    .mount("/", routes![
        routes::register, 
        routes::login, 
        routes::add_interest, 
        routes::get_interest
        ])

}
