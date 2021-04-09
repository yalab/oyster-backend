use super::schema::sites;

#[derive(Queryable)]

pub struct Site {
    pub id: i32,
    pub url: String,
    pub crawled_at: Option<std::time::SystemTime>,
}

#[derive(Insertable)]
#[table_name = "sites"]
pub struct NewSite<'a> {
    pub url: &'a str,
}
