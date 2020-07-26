use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Search {
    pub data: Vec<Data>,
    pub pagination: Pagination,
    pub meta: Meta,
}

#[derive(Deserialize, Debug)]
pub struct Random {
    pub data: Data,
    pub meta: Meta,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub r#type: String,
    pub id: String,
    pub url: String,
    pub slug: String,
    pub bitly_gif_url: String,
    pub bitly_url: String,
    pub embed_url: String,
    pub username: String,
    pub source: String,
    pub title: String,
    pub rating: String,
    pub content_url: String,
    pub tags: Option<Vec<String>>,
    pub featured_tags: Option<Vec<String>>,
    pub user_tags: Option<Vec<String>>,
    pub source_tld: String,
    pub source_post_url: String,
    pub is_sticker: i8,
    pub import_datetime: String,
    pub trending_datetime: String,
    // images: String,
    // user: String,
    // analytics_response_payload: String,
    // analytics: String,
}

#[derive(Deserialize, Debug)]
pub struct Pagination {
    pub total_count: i32,
    pub count: i32,
    pub offset: i32,
}

#[derive(Deserialize, Debug)]
pub struct Meta {
    pub status: i32,
    pub msg: String,
    pub response_id: String,
}
