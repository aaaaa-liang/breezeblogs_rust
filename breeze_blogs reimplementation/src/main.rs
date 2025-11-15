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
        routes::set_interests,
        routes::get_interests,
        routes::get_blog_posts, 
        routes::set_email,
        routes::send_news_mails, 
        routes::logout,
        routes::check_session
    ])

}
