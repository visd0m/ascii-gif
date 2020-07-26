use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Search {
    pub weburl: Option<String>,
    pub results: Vec<Result>,
    pub next: String,
}

#[derive(Deserialize, Debug)]
pub struct Result {
    pub tags: Vec<String>,
    pub url: String,
    pub media: Vec<Media>,
    pub created: f64,
    pub shares: i32,
    pub itemurl: String,
    pub composite: Option<String>,
    pub hasaudio: bool,
    pub title: String,
    pub id: String,
}

#[derive(Deserialize, Debug)]
pub struct Media {
    pub nanomp4: MediaItem,
    pub nanowebm: MediaItem,
    pub tinygif: MediaItem,
    pub tinymp4: MediaItem,
    pub tinywebm: MediaItem,
    pub webm: MediaItem,
    pub gif: MediaItem,
    pub mp4: MediaItem,
    pub loopedmp4: MediaItem,
    pub mediumgif: MediaItem,
    pub nanogif: MediaItem,
}

#[derive(Deserialize, Debug)]
pub struct MediaItem {
    pub url: String,
    pub dims: Vec<i32>,
    pub duration: Option<f32>,
    pub preview: String,
    pub size: Option<i32>,
}
