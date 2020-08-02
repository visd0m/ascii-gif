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
    pub images: Images,
    // pub user: String,
    // pub analytics_response_payload: String,
    // pub analytics: String,
}

#[derive(Deserialize, Debug)]
pub struct Images {
    pub downsized_large: Image,
    pub fixed_height_small_still: Image,
    pub original: Image,
    pub fixed_height_downsampled: Image,
    pub downsized_still: Image,
    pub fixed_height_still: Image,
    pub downsized_medium: Image,
    pub downsized: Image,
    pub preview_webp: Image,
    pub original_mp4: Image,
    pub fixed_height_small: Image,
    pub fixed_height: Image,
    pub downsized_small: Image,
    pub preview: Image,
    pub fixed_width_downsampled: Image,
    pub fixed_width_small_still: Image,
    pub fixed_width_small: Image,
    pub original_still: Image,
    pub fixed_width_still: Image,
    pub looping: Image,
    pub fixed_width: Image,
    pub preview_gif: Image,
}

#[derive(Deserialize, Debug)]
pub struct Image {
    pub frames: Option<String>,
    pub hash: Option<String>,
    pub height: Option<String>,
    pub mp4: Option<String>,
    pub mp4_size: Option<String>,
    pub size: Option<String>,
    pub url: Option<String>,
    pub webp: Option<String>,
    pub webp_size: Option<String>,
    pub width: Option<String>,
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
