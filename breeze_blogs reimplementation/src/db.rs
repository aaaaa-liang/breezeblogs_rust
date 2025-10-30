use mysql::*;
use mysql::prelude::*;

pub fn establish_connection() -> Result<PooledConn> {
    // connection string (use your encoded password)
    let url = "mysql://root:Hem%40ngi1234@localhost:3306/breeze_blogs";

    // create a connection pool
    let pool = Pool::new(url)?;

    // get a single connection from the pool
    let conn = pool.get_conn()?;

    println!("✅ Successfully connected to the MySQL database!");

    Ok(conn)
}
