// use sesame_rocket::rocket::{FromPConRequest, PConRequest, PConRequestOutcome};
// use sesame::{pcon::PCon, policy::NoPolicy};
// use sesame::context::Context;
// use sesame::SesameType;
// use sesame_mysql::{SesameConn, PConOpts};
// use std::sync::{Arc, Mutex};

// #[derive(SesameType, Clone)]
// pub struct BreezeContextData {
//     pub user: Option<PCon<String, NoPolicy>>,
//     pub db: Arc<Mutex<SesameConn>>,
// }

// pub type BreezeGuard = Context<BreezeContextData>;

// #[rocket::async_trait]
// impl<'a, 'r> FromPConRequest<'a, 'r> for BreezeContextData {
//     type PConError = ();

//     async fn from_pcon_request(req: PConRequest<'a, 'r>) -> PConRequestOutcome<Self, Self::PConError> {
//         let mut db = SesameConn::new(
//             PConOpts::from_url("mysql://root:annisnotanna!@127.0.0.1/").unwrap(),
//         ).unwrap();
//         db.query_drop("USE breeze_blogs").unwrap();

//         let user = None;

//         let data = BreezeContextData {
//             user,
//             db: Arc::new(Mutex::new(db)),
//         };
//         PConRequestOutcome::Success(data)
//     }
// }


use sesame_rocket::rocket::{FromPConRequest, PConRequest, PConRequestOutcome};
use sesame::{pcon::PCon, policy::NoPolicy};
use sesame::context::Context;
use sesame::SesameType;

#[derive(SesameType, Clone)]
pub struct BreezeContextData {
    pub user: Option<PCon<String, NoPolicy>>,
}

pub type BreezeGuard = Context<BreezeContextData>;

#[rocket::async_trait]
impl<'a, 'r> FromPConRequest<'a, 'r> for BreezeContextData {
    type PConError = ();
    
    async fn from_pcon_request(req: PConRequest<'a, 'r>) -> PConRequestOutcome<Self, Self::PConError> {
        let user = None;
        let data = BreezeContextData { user };
        PConRequestOutcome::Success(data)
    }
}
