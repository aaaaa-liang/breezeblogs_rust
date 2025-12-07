use sesame_mysql::{PConOpts, SesameConn};
use std::result::Result;

pub struct MySqlBackend {
    pub handle: SesameConn,
    _db_user: String,
    _db_password: String,
    _db_name: String,
}

impl MySqlBackend {
    pub fn new(user: &str, password: &str, dbname: &str) -> Result<Self, String> {
        let mut db = SesameConn::new(
            PConOpts::from_url(&format!("mysql://{}:YOURPASSWORD@127.0.0.1/", user, password)).unwrap(),
        ).unwrap();

        assert_eq!(db.ping(), true);
        db.query_drop(format!("USE {};", dbname)).unwrap();

        Ok(MySqlBackend {
            handle: db,
            _db_user: user.to_string(),
            _db_password: password.to_string(),
            _db_name: dbname.to_string(),
        })
    }
}
