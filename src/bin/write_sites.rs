extern crate diesel;
extern crate oyster;

use self::oyster::*;
use std::io::{stdin};

fn main() {
    let conection = establish_connection();
    println!("What would you like your title to be?");
    let mut url = String::new();
    stdin().read_line(&mut url).unwrap();
    let url = &url[..(url.len() - 1)];
    
    let site = create_site(&conection, url);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

