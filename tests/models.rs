use diesel::prelude::*;
use oyster::models::*;
use oyster::schema::sites::dsl::*;
use std::collections::HashMap;
use tokio;

#[test]
fn download_test() {
    let connection = oyster::establish_connection();
    let r = sites.find(1).get_result::<Site>(&connection);
    let site = match r {
        Ok(site) => site,
        Err(_)   => panic!("No such site"),
    };

    let get =  async {
      let resp = reqwest::get("https://httpbin.org/ip");
      println!("{:#?}", resp.await);
    };
    tokio::runtime::Runtime::new().unwrap().block_on(get);
    println!("Site {}", site.url);
}
