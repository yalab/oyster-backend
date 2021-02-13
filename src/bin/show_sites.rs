extern crate diesel;

use diesel::prelude::*;
use oyster::establish_connection;
use oyster::models::*;
use oyster::schema::sites::dsl::*;

fn main() {
    let connection = establish_connection();
    let results = sites
        .load::<Site>(&connection)
        .expect("Error loading sites");

    println!("Displaying {} sites", results.len());
    for site in results {
        println!("{}", site.url);
    }
}
