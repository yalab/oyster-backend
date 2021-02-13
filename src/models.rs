#[derive(Queryable)]

pub struct Site {
    pub id: i32,
    pub url: String,
    pub crawled_at: Option<std::time::SystemTime>,
}
