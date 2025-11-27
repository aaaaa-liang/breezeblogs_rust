use mysql::*;
use mysql::prelude::*;

pub fn establish_connection() -> Result<PooledConn> {
    // connection string components
    let opts = OptsBuilder::new()
        .ip_or_hostname(Some("localhost"))
        .tcp_port(3306)
        .user(Some("root"))
        .pass(Some("annisnotanna66!"))
        .db_name(Some("breeze_blogs"));

    // create a connection pool
    let pool = Pool::new(opts)?;

    // get a single connection from the pool
    let conn = pool.get_conn()?;

    println!("✅ Successfully connected to the MySQL database!");

    Ok(conn)
}
