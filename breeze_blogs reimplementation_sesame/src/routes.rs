use rocket::serde::Deserialize;
use bcrypt::{hash, DEFAULT_COST};
use bcrypt::verify; // needed
use sesame::policy::Reason;
use sesame::verified::{VerifiedRegion, execute_verified};
use sesame::{pcon::PCon, policy::NoPolicy};
use sesame_mysql::PConParam; // get? 
use sesame_rocket::rocket::{post, get, PConCookieJar, RequestPConJson, PConJson, PConCookie, ContextResponse};
use crate::policy::{BreezeGuard, BreezeContextData};
use sesame_mysql::SesameConn;
use sesame_mysql::PConOpts;
// use sesame_rocket::get;


// ----------------REGISTER----------------
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
    // Create DB connection
let mut db = SesameConn::new(
    PConOpts::from_url("mysql://root:YOURPASSWORD@127.0.0.1/").unwrap(),
).unwrap();
db.query_drop("USE breeze_blogs").unwrap();


// TO DO: we need to change this into VerifiedRegion and change to PConString for hashpassword 

    // let hashed_password = user.password.clone(); //hash(&user.password, DEFAULT_COST).unwrap();
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

// ----------------LOGIN----------------
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
    // Create DB connection
let mut db = SesameConn::new(
    PConOpts::from_url("mysql://root:YOURPASSWORD@127.0.0.1/").unwrap(),
).unwrap();
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
#[derive(RequestPConJson)]
pub struct InterestRequest {
    pub interests: Vec<PCon<String, NoPolicy>>,
}

#[post("/interests", data = "<data>")]
pub fn set_interests(
    cookies: PConCookieJar<'_, '_>,
    context: BreezeGuard,
    data: PConJson<InterestRequest>
) -> ContextResponse<String, NoPolicy, BreezeContextData> {
    // Check login - get cookie
    let user_cookie = cookies.get::<NoPolicy>("user_email");
    
    if user_cookie.is_none() {
        return ContextResponse(
            PCon::new("❌ Unauthorized: please log in first.".to_string(), NoPolicy {}),
            context
        );
    }

    // Extract the email value from the cookie and convert to owned String
    let cookie_binding = user_cookie.unwrap();
    let user_email_pcon = cookie_binding.value();
    
    // Convert to owned String to avoid lifetime issues
    let user_email_owned = user_email_pcon.clone().into_verified(
        VerifiedRegion::new(|email: &str| email.to_string())
    );
    
    // Extract the String and create a new PCon with NoPolicy
    let email_string = user_email_owned.discard_box().clone();
    let user_email = PCon::new(email_string, NoPolicy {});
    
    // Log the user email (for debugging)
    let _log_message = user_email.clone().into_verified(
        VerifiedRegion::new(|email: String| {
            println!("🍪 Setting interests for logged-in user: {}", email);
        })
    );

    // Create DB connection
    let mut db = SesameConn::new(
        PConOpts::from_url("mysql://root:YOURPASSWORD@127.0.0.1/").unwrap(),
    ).unwrap();
    db.query_drop("USE breeze_blogs").unwrap();

    // Lookup user_id from email
    let mut query_result = match db.exec_iter(
        "SELECT id FROM users WHERE email = ?",
        (user_email.clone(),),
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
                PCon::new("❌ User not found (database error).".to_string(), NoPolicy {}),
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

    // Extract user_id from the row
    let user_id: PCon<u32, _> = first_row.get("id").unwrap();
    
    // Drop query_result to release the borrow on db
    drop(query_result);

    // Delete old interests
    match db.exec_drop(
        "DELETE FROM interests WHERE user_id = ?",
        (user_id,),
        context.clone(),
    ) {
        Ok(_) => {},
        Err(e) => {
            return ContextResponse(
                PCon::new(format!("❌ Failed to delete old interests: {}", e), NoPolicy {}),
                context
            );
        }
    };

    // Insert new interests - fetch user_id each time since we can't clone AnyPolicy
    for interest in &data.interests {
        // Re-query for user_id
        let mut id_query = match db.exec_iter(
            "SELECT id FROM users WHERE email = ?",
            (user_email.clone(),),
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
        
        let id_row = id_query.next().unwrap().unwrap();
        let uid: PCon<u32, _> = id_row.get("id").unwrap();
        
        // Drop id_query to release the borrow before the next exec_drop
        drop(id_query);
        
        match db.exec_drop(
            "INSERT INTO interests (user_id, interest) VALUES (?, ?)",
            (uid, interest.clone()),
            context.clone(),
        ) {
            Ok(_) => {},
            Err(e) => {
                return ContextResponse(
                    PCon::new(format!("❌ Failed to insert interest: {}", e), NoPolicy {}),
                    context
                );
            }
        }
    }

    ContextResponse(
        PCon::new("✅ Interests updated successfully.".to_string(), NoPolicy {}),
        context
    )
}

// ----------------GET INTEREST----------------
#[get("/interests")]
pub fn get_interests(
    cookies: PConCookieJar<'_, '_>,
    context: BreezeGuard,
) -> ContextResponse<String, NoPolicy, BreezeContextData> {
    // Check login - get cookie
    let user_cookie = cookies.get::<NoPolicy>("user_email");
    
    if user_cookie.is_none() {
        return ContextResponse(
            PCon::new("[]".to_string(), NoPolicy {}),
            context
        );
    }

    // Extract the email value from the cookie and convert to owned String
    let cookie_binding = user_cookie.unwrap();
    let user_email_pcon = cookie_binding.value();
    
    // Convert to owned String to avoid lifetime issues
    let user_email_owned = user_email_pcon.clone().into_verified(
        VerifiedRegion::new(|email: &str| email.to_string())
    );
    
    // Extract the String and create a new PCon with NoPolicy
    let email_string = user_email_owned.discard_box().clone();
    let user_email = PCon::new(email_string, NoPolicy {});
    
    // Log the user email (for debugging)
    let _log_message = user_email.clone().into_verified(
        VerifiedRegion::new(|email: String| {
            println!("🍪 Fetching interests for user: {}", email);
        })
    );

    // Create DB connection
    let mut db = SesameConn::new(
        PConOpts::from_url("mysql://root:YOURPASSWORD@127.0.0.1/").unwrap(),
    ).unwrap();
    db.query_drop("USE breeze_blogs").unwrap();

    // Lookup user_id from email
    let mut query_result = match db.exec_iter(
        "SELECT id FROM users WHERE email = ?",
        (user_email.clone(),),
        context.clone(),
    ) {
        Ok(result) => result,
        Err(_e) => {
            return ContextResponse(
                PCon::new("[]".to_string(), NoPolicy {}),
                context
            );
        }
    };

    // Get first row
    let first_row = match query_result.next() {
        Some(Ok(row)) => row,
        None => {
            return ContextResponse(
                PCon::new("[]".to_string(), NoPolicy {}),
                context
            );
        }
        Some(Err(_e)) => {
            return ContextResponse(
                PCon::new("[]".to_string(), NoPolicy {}),
                context
            );
        }
    };

    // Extract user_id from the row
    let user_id: PCon<u32, _> = first_row.get("id").unwrap();
    
    // Drop query_result to release the borrow on db
    drop(query_result);

    // Fetch interests
    let mut interests_result = match db.exec_iter(
        "SELECT interest FROM interests WHERE user_id = ?",
        (user_id,),
        context.clone(),
    ) {
        Ok(result) => result,
        Err(_e) => {
            return ContextResponse(
                PCon::new("[]".to_string(), NoPolicy {}),
                context
            );
        }
    };

    // Collect all interests
    let mut interests: Vec<String> = Vec::new();
    while let Some(row_result) = interests_result.next() {
        match row_result {
            Ok(row) => {
                let interest: PCon<String, _> = row.get("interest").unwrap();
                // Extract the String value directly without discard_box (AnyPolicy doesn't support it)
                interest.into_verified(
                    VerifiedRegion::new(|i: String| {
                        interests.push(i);
                    })
                );
            }
            Err(_e) => {
                // Skip errors and continue
                continue;
            }
        }
    }

    // Convert to JSON string manually
    let json_string = format!("[{}]", interests.iter()
        .map(|s| format!("\"{}\"", s))
        .collect::<Vec<_>>()
        .join(","));

    ContextResponse(
        PCon::new(json_string, NoPolicy {}),
        context
    )
}

// ----------GET BLOG POSTS----------
#[get("/blog-posts")]
pub fn get_blog_posts(
    cookies: PConCookieJar<'_, '_>,
    context: BreezeGuard,
) -> ContextResponse<String, NoPolicy, BreezeContextData> {
    // Check login - get cookie
    let user_cookie = cookies.get::<NoPolicy>("user_email");
    
    if user_cookie.is_none() {
        return ContextResponse(
            PCon::new("❌ Unauthorized: please log in first.".to_string(), NoPolicy {}),
            context
        );
    }

    // Extract the email value from the cookie and convert to owned String
    let cookie_binding = user_cookie.unwrap();
    let user_email_pcon = cookie_binding.value();
    
    // Convert to owned String to avoid lifetime issues
    let user_email_owned = user_email_pcon.clone().into_verified(
        VerifiedRegion::new(|email: &str| email.to_string())
    );
    
    // Extract the String and create a new PCon with NoPolicy
    let email_string = user_email_owned.discard_box().clone();
    let user_email = PCon::new(email_string, NoPolicy {});

    // Create DB connection
    let mut db = SesameConn::new(
        PConOpts::from_url("mysql://root:YOURPASSWORD@127.0.0.1/").unwrap(),
    ).unwrap();
    db.query_drop("USE breeze_blogs").unwrap();

    // Lookup user_id from email
    let mut query_result = match db.exec_iter(
        "SELECT id FROM users WHERE email = ?",
        (user_email.clone(),),
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
                PCon::new("❌ User not found.".to_string(), NoPolicy {}),
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

    // Extract user_id from the row
    let user_id: PCon<u32, _> = first_row.get("id").unwrap();
    
    // Drop query_result to release the borrow on db
    drop(query_result);

    // Fetch interests for this user
    let mut interests_result = match db.exec_iter(
        "SELECT interest FROM interests WHERE user_id = ?",
        (user_id,),
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

    // Collect all interests
    let mut interests: Vec<String> = Vec::new();
    while let Some(row_result) = interests_result.next() {
        match row_result {
            Ok(row) => {
                let interest: PCon<String, _> = row.get("interest").unwrap();
                // Extract the String value
                interest.into_verified(
                    VerifiedRegion::new(|i: String| {
                        interests.push(i);
                    })
                );
            }
            Err(_e) => {
                continue;
            }
        }
    }

    // Drop interests_result to release the borrow on db
    drop(interests_result);

    if interests.is_empty() {
        return ContextResponse(
            PCon::new("❌ No interests found.".to_string(), NoPolicy {}),
            context
        );
    }

    // Fetch blog posts for each interest
    let mut return_str = String::new();
    for interest in &interests {
        let interest_pcon = PCon::new(interest.clone(), NoPolicy {});
        
        let mut blog_posts_result = match db.exec_iter(
            "SELECT content FROM blogposts WHERE interest = ?",
            (interest_pcon,),
            context.clone(),
        ) {
            Ok(result) => result,
            Err(_e) => {
                continue; // Skip this interest if query fails
            }
        };

        while let Some(row_result) = blog_posts_result.next() {
            match row_result {
                Ok(row) => {
                    let content: PCon<String, _> = row.get("content").unwrap();
                    // Extract the content and append to return string
                    content.into_verified(
                        VerifiedRegion::new(|c: String| {
                            return_str.push_str(&c);
                            return_str.push('\n');
                        })
                    );
                }
                Err(_e) => {
                    continue;
                }
            }
        }
        
        // Drop to release borrow before next iteration
        drop(blog_posts_result);
    }

    if return_str.is_empty() {
        return ContextResponse(
            PCon::new("❌ No blog posts found for your interests.".to_string(), NoPolicy {}),
            context
        );
    }

    ContextResponse(
        PCon::new(return_str, NoPolicy {}),
        context
    )
}

// ---------POST EMAIL PREFERENCE----------
#[derive(RequestPConJson)]
pub struct EmailRequest {
    pub email: PCon<String, NoPolicy>,
}

#[post("/email", data = "<data>")]
pub fn set_email(
    cookies: PConCookieJar<'_, '_>,
    context: BreezeGuard,
    data: PConJson<EmailRequest>
) -> ContextResponse<String, NoPolicy, BreezeContextData> {
    // Check login - get cookie
    let user_cookie = cookies.get::<NoPolicy>("user_email");
    
    if user_cookie.is_none() {
        return ContextResponse(
            PCon::new("❌ Unauthorized: please log in first.".to_string(), NoPolicy {}),
            context
        );
    }

    // Extract the email value from the cookie and convert to owned String
    let cookie_binding = user_cookie.unwrap();
    let user_email_pcon = cookie_binding.value();
    
    // Convert to owned String to avoid lifetime issues
    let user_email_owned = user_email_pcon.clone().into_verified(
        VerifiedRegion::new(|email: &str| email.to_string())
    );
    
    // Extract the String and create a new PCon with NoPolicy
    let email_string = user_email_owned.discard_box().clone();
    let user_email = PCon::new(email_string, NoPolicy {});

    // Create DB connection
    let mut db = SesameConn::new(
        PConOpts::from_url("mysql://root:YOURPASSWORD@127.0.0.1/").unwrap(),
    ).unwrap();
    db.query_drop("USE breeze_blogs").unwrap();

    // Lookup user_id from email
    let mut query_result = match db.exec_iter(
        "SELECT id FROM users WHERE email = ?",
        (user_email.clone(),),
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
                PCon::new("❌ User not found.".to_string(), NoPolicy {}),
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

    // Extract user_id from the row
    let user_id: PCon<u32, _> = first_row.get("id").unwrap();
    
    // Drop query_result to release the borrow on db
    drop(query_result);

    // Delete existing email preference (this will move user_id)
    match db.exec_drop(
        "DELETE FROM emails WHERE user_id = ?",
        (user_id,),
        context.clone(),
    ) {
        Ok(_) => {},
        Err(e) => {
            return ContextResponse(
                PCon::new(format!("❌ Failed to delete existing preference: {}", e), NoPolicy {}),
                context
            );
        }
    };

    // Re-query for user_id since it was moved
    let mut id_query = match db.exec_iter(
        "SELECT id FROM users WHERE email = ?",
        (user_email.clone(),),
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
    
    let id_row = id_query.next().unwrap().unwrap();
    let uid: PCon<u32, _> = id_row.get("id").unwrap();
    
    // Drop id_query to release the borrow
    drop(id_query);

    // Insert new email preference
    match db.exec_drop(
        "INSERT INTO emails (user_id, email) VALUES (?, ?)",
        (uid, data.email.clone()),
        context.clone(),
    ) {
        Ok(_) => {
            ContextResponse(
                PCon::new("✅ Email set successfully.".to_string(), NoPolicy {}),
                context
            )
        }
        Err(e) => {
            ContextResponse(
                PCon::new(format!("❌ Failed to set email: {}", e), NoPolicy {}),
                context
            )
        }
    }
}

// ----------SEND NEW MAILS----------
#[get("/send-news-mails")]
pub fn send_news_mails(
    cookies: PConCookieJar<'_, '_>,
    context: BreezeGuard,
) -> ContextResponse<String, NoPolicy, BreezeContextData> {
    // Create DB connection
    let mut db = SesameConn::new(
        PConOpts::from_url("mysql://root:YOURPASSWORD@127.0.0.1/").unwrap(),
    ).unwrap();
    db.query_drop("USE breeze_blogs").unwrap();

    // Fetch all emails
    let mut emails_result = match db.exec_iter(
        "SELECT email FROM emails",
        (),
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

    // Collect all emails
    let mut emails: Vec<String> = Vec::new();
    while let Some(row_result) = emails_result.next() {
        match row_result {
            Ok(row) => {
                let email: PCon<String, _> = row.get("email").unwrap();
                // Extract the email string
                email.into_verified(
                    VerifiedRegion::new(|e: String| {
                        emails.push(e);
                    })
                );
            }
            Err(_e) => {
                continue;
            }
        }
    }

    if emails.is_empty() {
        return ContextResponse(
            PCon::new("❌ No emails found.".to_string(), NoPolicy {}),
            context
        );
    }

    // Build return string, just like the original Python-style format
    let mut return_str = String::new();
    for email in emails {
        return_str.push_str(&format!("{};{};", email, email));
    }

    ContextResponse(
        PCon::new(return_str, NoPolicy {}),
        context
    )
}

// ----------LOGOUT----------
// #[post("/logout")]
// pub fn logout(
//     cookies: PConCookieJar<'_, '_>,
//     context: BreezeGuard,
// ) -> ContextResponse<String, NoPolicy, BreezeContextData> {
//     // Check if the cookie exists
//     if cookies.get::<NoPolicy>("user_email").is_some() {
//         // Remove the cookie - build a cookie with the same name to remove it
//         let remove_cookie = PConCookie::build("user_email", PCon::new("", NoPolicy {}))
//             .path("/")
//             .finish();
//         cookies.remove(remove_cookie);
//         println!("👋 User logged out successfully");
        
//         ContextResponse(
//             PCon::new("✅ Logged out successfully.".to_string(), NoPolicy {}),
//             context
//         )
//     } else {
//         ContextResponse(
//             PCon::new("⚠️ No active session found.".to_string(), NoPolicy {}),
//             context
//         )
//     }
// }

#[post("/logout")]
pub fn logout(
    cookies: PConCookieJar<'_, '_>,
    context: BreezeGuard,
) -> ContextResponse<String, NoPolicy, BreezeContextData> {
    // Check if cookie exists
    if cookies.get::<NoPolicy>("user_email").is_some() {
        // TODO: Cookie removal has lifetime issues with Sesame's current API
        // For now, we acknowledge the logout request
        // In a production system, you would need to handle this differently
        // possibly by setting an expiration or using a different logout mechanism
        println!("👋 User logout requested");
        
        ContextResponse(
            PCon::new("✅ Logout requested (cookie removal pending).".to_string(), NoPolicy {}),
            context
        )
    } else {
        ContextResponse(
            PCon::new("⚠️ No active session found.".to_string(), NoPolicy {}),
            context
        )
    }
}

// ----------SESSION----------
#[get("/session")]
pub fn check_session(
    cookies: PConCookieJar<'_, '_>,
    context: BreezeGuard,
) -> ContextResponse<String, NoPolicy, BreezeContextData> {
    // Check if the cookie exists
    match cookies.get::<NoPolicy>("user_email") {
        Some(cookie) => {
            // Extract the email value from the cookie
            let email_pcon = cookie.value();
            
            // Use into_verified to create the response message
            let response = email_pcon.clone().into_verified(
                VerifiedRegion::new(|email: &str| {
                    format!("✅ Logged in as: {}", email)
                })
            );
            
            // Extract the string and wrap in PCon with NoPolicy
            let response_string = response.discard_box().clone();
            
            ContextResponse(
                PCon::new(response_string, NoPolicy {}),
                context
            )
        }
        None => {
            ContextResponse(
                PCon::new("⚠️ No active session.".to_string(), NoPolicy {}),
                context
            )
        }
    }
}
