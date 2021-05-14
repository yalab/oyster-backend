use super::schema::sites;
use html5ever::tendril::TendrilSink;
use html5ever::parse_document;
use markup5ever_rcdom::RcDom;

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
    pub fn parse(&self, str: String) -> String{
        let dom: RcDom = parse_document(RcDom::default(), Default::default()).one(str.clone());
        return str;
    }
}

#[derive(Insertable)]
#[table_name = "sites"]
pub struct NewSite<'a> {
    pub url: &'a str,
}
