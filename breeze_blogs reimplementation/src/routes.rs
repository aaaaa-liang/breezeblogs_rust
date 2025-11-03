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

// ---------- POST INTERESTS ----------
#[derive(Deserialize)]
pub struct InterestRequest {
    pub username: String,
    pub interest: String,
}

#[post("/interests", data = "<data>")]
pub fn add_interest(data: Json<InterestRequest>) -> String {
    let mut conn = db::establish_connection().expect("DB connection failed");

    // Lookup user_id by username
    let user_id: Option<u32> = conn.exec_first(
        "SELECT id FROM users WHERE username = :username",
        params! { "username" => &data.username }
    ).unwrap();

    let user_id = match user_id {
        Some(id) => id,
        None => return format!("❌ No user found with username '{}'", data.username),
    };

    // Insert interest with user_id
    let result = conn.exec_drop(
        "INSERT INTO interests (user_id, interest) VALUES (:user_id, :interest)",
        params! {
            "user_id" => user_id,
            "interest" => &data.interest,
        },
    );

    match result {
        Ok(_) => format!("✅ Interest '{}' added for {}", data.interest, data.username),
        Err(e) => format!("❌ Failed to add interest: {}", e),
    }
}

// ---------- GET INTERESTS ----------
#[derive(Deserialize)]
pub struct GetInterestRequest {
    pub username: String,
}

#[get("/interests/<username>")]
pub fn get_interest(username: String) -> Json<Vec<String>> {
    let mut conn = db::establish_connection().expect("DB connection failed");

    // Join interests with users to fetch interests by username
    let interests: Vec<String> = conn.exec_map(
        "SELECT i.interest
         FROM interests i
         JOIN users u ON i.user_id = u.id
         WHERE u.username = :username",
        params! { "username" => &username },
        |interest: String| interest
    ).unwrap_or_else(|_| vec![]);

    Json(interests)
}
