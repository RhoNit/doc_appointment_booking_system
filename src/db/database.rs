use diesel::{Connection, PgConnection};
use dotenv;


pub fn db_connection() -> PgConnection {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DB_URL must be set");
    PgConnection::establish(&database_url).expect("Failed while connecting to the database")
}