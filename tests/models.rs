use diesel::prelude::*;
use oyster::models::*;
use oyster::schema::sites::dsl::*;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[test]
fn download_test() {
    let connection = oyster::establish_connection();
    let r = sites.find(1).get_result::<Site>(&connection);
    let site = match r {
        Ok(site) => site,
        Err(_)   => panic!("No such site"),
    };
    let response = site.get();
    println!("{:#?}", response);
    let m = async {
        return response.text().await
    };
    let body = match tokio::runtime::Runtime::new().unwrap().block_on(m){
        Ok(body) => body,
        Err(_) => panic!("Error body")
    };
    println!("{:#?}", body);

    let path = Path::new("test.html");
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(body.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
