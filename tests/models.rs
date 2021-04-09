use diesel::prelude::*;
use oyster::models::*;
use oyster::schema::sites::dsl::*;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[test]
fn download_test() {
    let connection = oyster::establish_connection();
    let r = sites.find(1).get_result::<Site>(&connection).unwrap();
    let m = async {
        return reqwest::get(r.url).await.unwrap().text().await;
    };
    let body = tokio::runtime::Runtime::new().unwrap().block_on(m);
    let path = Path::new("test.html");
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(body.unwrap().as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
