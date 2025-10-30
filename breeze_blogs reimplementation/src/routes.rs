use rocket::serde::{Deserialize, json::Json};
use mysql::*;
use mysql::prelude::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::db;

// ---------- REGISTER ----------
#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[post("/register", data = "<user>")]
pub fn register(user: Json<RegisterRequest>) -> String {
    let mut conn = db::establish_connection().expect("DB connection failed");

    let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();

    let result = conn.exec_drop(
        "INSERT INTO users (username, email, password) VALUES (:username, :email, :password)",
        params! {
            "username" => &user.username,
            "email" => &user.email,
            "password" => &hashed_password,
        }
    );

    match result {
        Ok(_) => format!("✅ User '{}' registered successfully!", user.username),
        Err(e) => format!("❌ Failed to register user: {}", e),
    }
}

// ---------- LOGIN ----------
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[post("/login", data = "<user>")]
pub fn login(user: Json<LoginRequest>) -> String {
    let mut conn = db::establish_connection().expect("DB connection failed");

    let selected_user: Option<(String, String)> = conn.exec_first(
        "SELECT email, password FROM users WHERE email = :email",
        params! {
            "email" => &user.email,
        },
    ).unwrap();

    match selected_user {
        Some((email, stored_hash)) => {
            if verify(&user.password, &stored_hash).unwrap() {
                format!("✅ Login successful for '{}'", email)
            } else {
                "❌ Invalid password.".to_string()
            }
        }
        None => "❌ No account found with that email.".to_string(),
    }
}
