use diesel::prelude::*;
use oyster::models::*;
use oyster::schema::sites::dsl::*;

#[test]
fn download_test() {
    let connection = oyster::establish_connection();
    let r = sites.find(1).get_result::<Site>(&connection);
    let site = match r {
        Ok(site) => site,
        Err(_)   => panic!("No such site"),
    };
    println!("Site {}", site.url);
}
