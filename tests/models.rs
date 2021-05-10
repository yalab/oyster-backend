use diesel::prelude::*;
use oyster::models::*;
use oyster::schema::sites::dsl::*;

#[test]
fn download_test() {
    let connection = oyster::establish_connection();
    let site = sites.find(1).get_result::<Site>(&connection).unwrap();
    let body = site.fetch();
    assert_eq!(&body[0..15], "<!DOCTYPE html>");
    let len = body.len();
    assert_eq!(&body[(len - 9)..(len - 2)], "</html>");
}

#[test]
fn parse_test() {
    let site = Site{id: 0, url: String::new(), crawled_at: None};
    let body = std::fs::read_to_string("tests/fixtures/files/sumo.jp.html").unwrap();
    site.parse(body);
}