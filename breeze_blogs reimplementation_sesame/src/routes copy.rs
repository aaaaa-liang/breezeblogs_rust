use rocket::serde::Deserialize;
use bcrypt::{hash, verify, DEFAULT_COST};

use sesame::verified::{VerifiedRegion};
use sesame::{pcon::PCon, policy::NoPolicy};

use sesame_mysql::{PConOpts, SesameConn};
use sesame_mysql::PConParam; 
use sesame_mysql::PConRow;
use sesame::policy::AnyPolicy;
use sesame_rocket::rocket::{post, PConCookieJar, RequestPConJson, PConJson, PConCookie, ContextResponse};
use crate::policy::{BreezeGuard, BreezeContextData};

// ---------- REGISTER ----------

#[derive(RequestPConJson)]
pub struct RegisterRequest {
    pub username: PCon<String, NoPolicy>,
    pub email: PCon<String, NoPolicy>,
    pub password: PCon<String, NoPolicy>,
}

#[post("/register", data = "<user>")]
pub fn register(
    cookies: PConCookieJar<'_, '_>,
    context: BreezeGuard,
    user: PConJson<RegisterRequest>
) -> ContextResponse<String, NoPolicy, BreezeContextData> {
    let mut db = SesameConn::new(PConOpts::from_url("mysql://root:annisnotanna66!@127.0.0.1/").unwrap(),).unwrap();
        db.query_drop("USE breeze_blogs").unwrap();

    let hashed_password: PCon<String, NoPolicy> = user.password.clone().into_verified(VerifiedRegion::new(|password: String| {
        hash(&password, DEFAULT_COST).unwrap()
    }));
    let email_pcon = user.email.clone();

    // exec_drop takes (query, params)
    let result = db.exec_drop(
        "INSERT INTO users (username, email, password) VALUES (?, ?, ?)",
        (user.username.clone(), email_pcon, hashed_password),
        context.clone(),
    );

    match result {
        Ok(_) => {
            cookies.add(
                PConCookie::build("user_email", user.email.clone())
                    .path("/")
                    .finish(),
                context.clone(),
            );

            let output: PCon<String, NoPolicy> = user.username.clone().into_verified(VerifiedRegion::new(|username: String| {
                format!("✅ User '{}' registered successfully!", username)
            }));

            //format!("✅ User '{}' registered successfully!", user.username)
            ContextResponse(output, context)
        }
        Err(e) => {
            ContextResponse(
                PCon::new(format!("❌ Failed to register user: {}", e), NoPolicy {}),
                context
            )
        }
    }
}

// ---------- LOGIN ----------
#[derive(RequestPConJson)]
pub struct LoginRequest {
    pub email: PCon<String, NoPolicy>,
    pub password: PCon<String, NoPolicy>,
}
#[post("/login", data = "<user>")]
pub fn login(
    cookies: PConCookieJar<'_, '_>,
    context: BreezeGuard,
    user: PConJson<LoginRequest>
) -> ContextResponse<String, NoPolicy, BreezeContextData> {
    let mut db = SesameConn::new(PConOpts::from_url("mysql://root:annisnotanna66!@127.0.0.1/").unwrap(),).unwrap();
        db.query_drop("USE breeze_blogs").unwrap();

    // Query database for user by email
    let mut query_result = match db.exec_iter(
        "SELECT email, password FROM users WHERE email = ?",
        (user.email.clone(),),
        context.clone(),
    ) {
        Ok(result) => result,
        Err(e) => {
            return ContextResponse(
                PCon::new(format!("❌ Database error: {}", e), NoPolicy {}),
                context
            );
        }
    };

    // Get first row
    let first_row = match query_result.next() {
        Some(Ok(row)) => row,
        None => {
            return ContextResponse(
                PCon::new("❌ No account found with that email.".to_string(), NoPolicy {}),
                context
            );
        }
        Some(Err(e)) => {
            return ContextResponse(
                PCon::new(format!("❌ Database error: {}", e), NoPolicy {}),
                context
            );
        }
    };

    // Extract stored password hash from row (THIS LINE WAS MISSING!)
    let stored_hash = first_row.get("password").unwrap();

    // Use into_verified to extract password string
    let password_str_pcon = user.password.clone().into_verified(
        VerifiedRegion::new(|pwd: String| pwd)
    );

    // Extract it as plain string (NoPolicy allows this)
    let input_password_str = password_str_pcon.as_ref().discard_box().clone();

    // Now verify password against stored hash
    let is_valid = stored_hash.into_verified(
        VerifiedRegion::new(move |stored_pwd: String| {
            verify(&input_password_str, &stored_pwd).unwrap_or(false)
        })
    );

    // Create success/failure message
    let response = user.email.clone().into_verified(
        VerifiedRegion::new(|email: String| {
            format!("✅ Login successful for '{}'", email)
        })
    );

    ContextResponse(response, context)
}

// ---------- POST INTERESTS ----------


