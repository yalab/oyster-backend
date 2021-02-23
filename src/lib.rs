#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use self::models::{NewSite, Site};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_site<'a>(conn: &PgConnection, url: &'a str) -> Site {
    use schema::sites;

    let new_site = NewSite { url: url };

    diesel::insert_into(sites::table)
        .values(&new_site)
        .get_result(conn)
        .expect("Error saving new post")
}

pub mod models;
pub mod schema;
