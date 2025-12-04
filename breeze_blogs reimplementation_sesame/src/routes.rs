use rocket::serde::Deserialize;
use bcrypt::{hash, DEFAULT_COST};

use sesame::verified::{VerifiedRegion};
use sesame::{pcon::PCon, policy::NoPolicy};
use sesame_mysql::PConParam; // get? 
use sesame_rocket::rocket::{post, PConCookieJar, RequestPConJson, PConJson, PConCookie, ContextResponse};
use crate::policy::{BreezeGuard, BreezeContextData};

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
    let binding = context.data().unwrap().db.clone();
    let mut db = binding.lock().unwrap();


    // TO DO: we need to change this into VerifiedRegion and change to PConString for hashpassword 

    let hashed_password = user.password.clone(); //hash(&user.password, DEFAULT_COST).unwrap();
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
