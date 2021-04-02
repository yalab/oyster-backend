use super::schema::sites;
use tokio;

#[derive(Queryable)]

pub struct Site {
    pub id: i32,
    pub url: String,
    pub crawled_at: Option<std::time::SystemTime>,
}

impl Site {
    pub fn get(&self) -> reqwest::Response {
        let url = &self.url;
        let m =  async {
            let result = reqwest::get(url).await;
            let response = match result {
                Ok(response) => response,
                Err(_) => panic!("Response Error"),
            };
            return response
        };
        return tokio::runtime::Runtime::new().unwrap().block_on(m)
    }
}

#[derive(Insertable)]
#[table_name = "sites"]
pub struct NewSite<'a> {
    pub url: &'a str,
}
