use rocket::serde::{Deserialize, json::Json};
use mysql::*;
use mysql::prelude::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::db;
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

#[post("/register", data = "<user>")]
pub fn register(user: Json<RegisterRequest>) -> String {
    let mut conn = db::establish_connection().expect("DB connection failed");

    let hashed_password = hash(&user.password, DEFAULT_COST).unwrap();

    let result = conn.exec_drop(
        "INSERT INTO users (username, email, password) VALUES (?, ?, ?)",
        (&user.username, &user.email, &hashed_password),
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
pub fn login(cookies: &CookieJar<'_>, user: Json<LoginRequest>) -> String {
    let mut conn = db::establish_connection().expect("DB connection failed");

    let selected_user: Option<(String, String)> = conn
        .exec_first("SELECT email, password FROM users WHERE email = ?", (&user.email,))
        .unwrap();

    match selected_user {
        Some((email, stored_hash)) => {
            if verify(&user.password, &stored_hash).unwrap() {
                cookies.add(
                    Cookie::build("user_email", email.to_string())
                        .path("/")
                        .finish(),
                );                
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
    pub interests: Vec<String>,
}

#[post("/interests", data = "<data>")]
pub fn set_interests(cookies: &CookieJar<'_>, data: Json<InterestRequest>) -> String {
    // Check login
    let user_cookie = cookies.get("user_email");
    if user_cookie.is_none() {
        return "❌ Unauthorized: please log in first.".to_string();
    }

    let user_email = user_cookie.unwrap().value().to_string();
    println!("🍪 Setting interests for logged-in user: {}", user_email);

    let mut conn = db::establish_connection().expect("DB connection failed");

    // Lookup user_id from email (NOT username anymore)
    let user_id: Option<u32> = conn.exec_first(
        "SELECT id FROM users WHERE email = ?",
        (user_email.clone(),),
    ).unwrap();

    let user_id = match user_id {
        Some(id) => id,
        None => return "❌ User not found (database error).".to_string(),
    };

    // Delete old interests
    conn.exec_drop("DELETE FROM interests WHERE user_id = ?", (user_id,))
        .unwrap();

    // Insert new interests
    for interest in &data.interests {
        conn.exec_drop(
            "INSERT INTO interests (user_id, interest) VALUES (?, ?)",
            (user_id, interest),
        ).unwrap();
    }

    "✅ Interests updated successfully.".to_string()
}

// ---------- GET INTERESTS ----------
#[get("/interests")]
pub fn get_interests(cookies: &CookieJar<'_>) -> Json<Vec<String>> {
    // Check login
    let user_cookie = cookies.get("user_email");
    if user_cookie.is_none() {
        return Json(vec![]);
    }

    let user_email = user_cookie.unwrap().value().to_string();
    println!("🍪 Fetching interests for user: {}", user_email);

    let mut conn = db::establish_connection().expect("DB connection failed");

    // Lookup user_id via email
    let user_id: Option<u32> = conn.exec_first(
        "SELECT id FROM users WHERE email = ?",
        (user_email.clone(),),
    ).unwrap();

    let user_id = match user_id {
        Some(id) => id,
        None => return Json(vec![]),
    };

    // Fetch interests
    let interests: Vec<String> = conn.exec_map(
        "SELECT interest FROM interests WHERE user_id = ?",
        (user_id,),
        |interest| interest,
    ).unwrap_or_else(|_| vec![]);

    Json(interests)
}

// ---------- GET BLOG POSTS ----------
#[get("/blog-posts")]
pub fn get_blog_posts(cookies: &CookieJar<'_>) -> (rocket::http::Status, String) {
    // Step 1: Check login
    let user_cookie = cookies.get("user_email");
    if user_cookie.is_none() {
        return (rocket::http::Status::Unauthorized, "❌ Unauthorized: please log in first.".to_string());
    }
    let user_email = user_cookie.unwrap().value().to_string();

    let mut conn = db::establish_connection().expect("DB connection failed");

    // Step 2: Lookup user_id from email
    let user_id: Option<u32> = conn.exec_first(
        "SELECT id FROM users WHERE email = ?",
        (user_email.clone(),),
    ).unwrap();

    let user_id = match user_id {
        Some(id) => id,
        None => return (rocket::http::Status::NotFound, "❌ User not found.".to_string()),
    };

    // Step 3: Fetch interests
    let interests: Vec<String> = conn.exec_map(
        "SELECT interest FROM interests WHERE user_id = ?",
        (user_id,),
        |interest| interest,
    ).unwrap_or_else(|_| vec![]);

    if interests.is_empty() {
        return (rocket::http::Status::NotFound, "❌ No interests found.".to_string());
    }

    // Step 4: Fetch blog posts for each interest
    let mut return_str = String::new();
    for interest in &interests {
        let blog_posts: Vec<String> = conn.exec_map(
            "SELECT content FROM blogposts WHERE interest = ?",
            (interest,),
            |content| content,
        ).unwrap_or_else(|_| vec![]);

        for post in blog_posts {
            return_str.push_str(&post);
            return_str.push('\n'); // Separate posts with newline
        }
    }

    if return_str.is_empty() {
        return (rocket::http::Status::NotFound, "❌ No blog posts found for your interests.".to_string());
    }

    (rocket::http::Status::Ok, return_str)
}


// ---------- POST EMAIL PREFERENCE ----------
#[derive(Deserialize)]
pub struct EmailRequest {
    pub email: String,
}

#[post("/email", data = "<data>")]
pub fn set_email(cookies: &CookieJar<'_>, data: Json<EmailRequest>) -> String {
    // Check login
    let user_cookie = cookies.get("user_email");
    if user_cookie.is_none() {
        return "❌ Unauthorized: please log in first.".to_string();
    }
    let user_email = user_cookie.unwrap().value().to_string();

    let mut conn = db::establish_connection().expect("DB connection failed");

    // Lookup user_id
    let user_id: Option<u32> = conn.exec_first(
        "SELECT id FROM users WHERE email = ?",
        (user_email,),
    ).unwrap();

    let user_id = match user_id {
        Some(id) => id,
        None => return "❌ User not found.".to_string(),
    };

    // Delete existing preference
    conn.exec_drop("DELETE FROM emails WHERE user_id = ?", (user_id,))
        .unwrap();

    // Insert new preference
    match conn.exec_drop(
        "INSERT INTO emails (user_id, email) VALUES (?, ?)",
        (user_id, &data.email),
    ) {
        Ok(_) => "✅ Email set successfully.".to_string(),
        Err(e) => format!("❌ Failed to set email: {}", e),
    }
}

// ---------- SEND NEW MAILS ----------
#[get("/send-news-mails")]
pub fn send_news_mails() -> (Status, String) {
    let mut conn = db::establish_connection().expect("DB connection failed");

    // Fetch all emails
    let emails: Vec<String> = conn.exec_map(
        "SELECT email FROM emails",
        (),
        |email| email,
    ).unwrap_or_else(|_| vec![]);

    if emails.is_empty() {
        return (Status::NotFound, "❌ No emails found.".to_string());
    }

    // Build return string, just like Python
    let mut return_str = String::new();
    for email in emails {
        return_str.push_str(&format!("{};{};", email, email));
    }

    (Status::Ok, return_str)
}

// ---------- LOGOUT ----------
#[post("/logout")]
pub fn logout(cookies: &rocket::http::CookieJar<'_>) -> String {
    // ✅ Step 3: remove the cookie if it exists
    if cookies.get("user_email").is_some() {
        cookies.remove(Cookie::named("user_email"));
        println!("👋 User logged out successfully");
        "✅ Logged out successfully.".to_string()
    } else {
        "⚠️ No active session found.".to_string()
    }
}

// ---------- SESSION ----------
#[get("/session")]
pub fn check_session(cookies: &rocket::http::CookieJar<'_>) -> String {
    match cookies.get("user_email") {
        Some(cookie) => {
            let email = cookie.value();
            format!("✅ Logged in as: {}", email)
        }
        None => "⚠️ No active session.".to_string(),
    }
}

