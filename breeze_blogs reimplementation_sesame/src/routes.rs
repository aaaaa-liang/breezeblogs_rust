use rocket::serde::{Deserialize, json::Json}; // 
use rocket::State;
use sesame_core::pcon::PCon; // sesame core 
use sesame_core::policy::NoPolicy; // sesame core 
use sesame_rocket::rocket::{post, PConResponseEnum}; // sesame rocket 
use sesame_rocket::PrivacyContext; // sesame rocket 


use mysql::*; // 
use mysql::prelude::*; // 
use bcrypt::{hash, verify, DEFAULT_COST}; // 
use crate::db;
use crate::policy::YouContext;
use rocket::http::Cookie;
use rocket::http::CookieJar;
use rocket::http::Status;


// ---------- REGISTER ----------
#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

// #[post("/register", data = "<user>")]
// pub fn register(user: Json<RegisterRequest>) -> String {
//     let mut conn = db::establish_connection().expect("DB connection failed");

//     let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();

//     let result = conn.exec_drop(
//         "INSERT INTO users (username, email, password) VALUES (?, ?, ?)",
//         (&user.username, &user.email, &hashed_password),
//     );

//     match result {
//         Ok(_) => format!("✅ User '{}' registered successfully!", user.username),
//         Err(e) => format!("❌ Failed to register user: {}", e),
//     }
// }


// #[post("/register", data = "<data>")]
// pub fn register(
//     data: PCon<Json<RegisterRequest>, NoPolicy>,
//     context: YouContext
// ) -> PConResponseEnum {

//     let user = data.into_inner();     // extract JSON
//     let mut conn = db::establish_connection().unwrap();

//     let hashed = hash(&user.password, DEFAULT_COST).unwrap();

//     let result = conn.exec_drop(
//         "INSERT INTO users (username, email, password) VALUES (?, ?, ?)",
//         (&user.username, &user.email, &hashed),
//     );

//     match result {
//         Ok(_) => format!("✅ User '{}' registered successfully!", user.username).into(),
//         Err(e) => format!("❌ Failed to register user: {}", e).into(),
//     }
// }

#[post("/register", data = "<user>")]
pub fn register(
    user: PCon<Json<RegisterRequest>, NoPolicy>,   // Sesame-wrapped request body
    context: Context,                               // Sesame privacy context
) -> PCon<String, NoPolicy> {                       // Sesame-wrapped return value
    // Extract the inner JSON from PCon
    let user = user.into_inner();

    let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();

    let mut conn = db::establish_connection().expect("DB connection failed");

    let result = conn.exec_drop(
        "INSERT INTO users (username, email, password) VALUES (?, ?, ?)",
        (&user.username, &user.email, &hashed_password),
    );

    let response = match result {
        Ok(_) => format!("✅ User '{}' registered successfully!", user.username),
        Err(e) => format!("❌ Failed to register user: {}", e),
    };

    // Wrap output in PCon (required by Sesame)
    PCon::new(response, context)
}
