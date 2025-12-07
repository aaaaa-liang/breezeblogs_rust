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
                routes::login,
                routes::set_interests,
                routes::get_interests,
                routes::get_blog_posts,
                routes::set_email,
                routes::send_news_mails,
                routes::logout,
                routes::check_session
            ],
        )
        .launch()
        .await
    {
        eprintln!("❌ Rocket failed to launch: {:?}", e);
    }
}
