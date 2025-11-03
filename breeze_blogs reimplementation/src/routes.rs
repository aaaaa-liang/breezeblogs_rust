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
    pub interests: Vec<String>, // multiple interests at once
}

#[post("/interests", data = "<data>")]
pub fn set_interests(data: Json<InterestRequest>) -> String {
    let mut conn = db::establish_connection().expect("DB connection failed");

    // Get user_id from username
    let user_id: Option<u32> = conn.exec_first(
        "SELECT id FROM users WHERE username = :username",
        params! { "username" => &data.username },
    ).unwrap();

    let user_id = match user_id {
        Some(id) => id,
        None => return format!("❌ No user found with username '{}'", data.username),
    };

    // Delete existing interests
    let _ = conn.exec_drop(
        "DELETE FROM interests WHERE user_id = :user_id",
        params! { "user_id" => user_id },
    );

    // Insert new interests
    for interest in &data.interests {
        let _ = conn.exec_drop(
            "INSERT INTO interests (user_id, interest) VALUES (:user_id, :interest)",
            params! { "user_id" => user_id, "interest" => interest },
        );
    }

    format!("✅ Interests set successfully for '{}'", data.username)
}

// ---------- GET INTERESTS ----------
#[get("/interests/<username>")]
pub fn get_interests(username: String) -> Json<Vec<String>> {
    let mut conn = db::establish_connection().expect("DB connection failed");

    // Get user_id from username
    let user_id: Option<u32> = conn.exec_first(
        "SELECT id FROM users WHERE username = :username",
        params! { "username" => &username },
    ).unwrap();

    let user_id = match user_id {
        Some(id) => id,
        None => return Json(vec![]), // user not found
    };

    // Fetch all interests
    let interests: Vec<String> = conn.exec_map(
        "SELECT interest FROM interests WHERE user_id = :user_id",
        params! { "user_id" => user_id },
        |interest| interest,
    ).unwrap_or_else(|_| vec![]);

    Json(interests)
}
