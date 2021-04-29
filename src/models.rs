use super::schema::sites;

#[derive(Queryable)]

pub struct Site {
    pub id: i32,
    pub url: String,
    pub crawled_at: Option<std::time::SystemTime>,
}

impl Site {
    pub fn fetch(&self) -> String {
       let m = async {
            return reqwest::get(&self.url).await.unwrap().text().await;
        };
        return tokio::runtime::Runtime::new().unwrap().block_on(m).unwrap();
    }
}

#[derive(Insertable)]
#[table_name = "sites"]
pub struct NewSite<'a> {
    pub url: &'a str,
}
