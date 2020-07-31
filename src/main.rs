use crate::ascii::frame::AsciiFrame;
use crate::ascii::gif::player::Player;
use crate::ascii::gif::AsciiGif;
use crate::http::get;
use gif::SetParameter;
use hyper::client::HttpConnector;
use hyper::Client;
use hyper_tls::HttpsConnector;
use std::env;
use url::form_urlencoded::byte_serialize;

pub mod ascii;
pub mod giphy;
pub mod http;
pub mod tenor;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Error loading env variables");
    let giphy_api_key = env::var("GIPHY_API_KEY").expect("Error loading giphy apikey");
    let tenor_api_key = env::var("TENOR_API_KEY").expect("Error loading tenor apikey");

    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    debug_assert_eq!(2, args.len());
    let q = args.get(1).unwrap();

    let (w, h) = term_size::dimensions().unwrap();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // let (_, _) = giphy(&client, q, giphy_api_key).await;
    let search_tenor = tenor(&client, q, tenor_api_key).await;

    let url = &search_tenor
        .results
        .first()
        .unwrap()
        .media
        .first()
        .unwrap()
        .nanogif
        .url;

    // let url =
    //    "https://media.tenor.com/images/a53a589ea59868ab7458d4006c080458/tenor.gif".to_string();

    let mut decoder = gif::Decoder::new(get(&client, &url).await.unwrap());
    decoder.set(gif::ColorOutput::RGBA);
    let mut decoder = decoder.read_info().unwrap();
    let global_width = decoder.width().clone();
    let global_height = decoder.height().clone();

    let mut frames: Vec<AsciiFrame> = Vec::new();
    while let Some(frame) = decoder.read_next_frame().unwrap() {
        frames.push(frame.into())
    }

    let ascii_gif = AsciiGif::new(frames, global_width, global_height);

    let mut player = Player::new(h as u16, w as u16);

    loop {
        player.play(&ascii_gif);
        dbg!(&url);
    }
}

async fn giphy(
    client: &Client<HttpsConnector<HttpConnector>>,
    q: &String,
    apikey: String,
) -> (giphy::types::Search, giphy::types::Random) {
    let giphy = giphy::Giphy::new(client, apikey);

    let search_giphy = giphy.search(q, 1).await;
    let random_giphy = giphy.random(q).await;

    (search_giphy.unwrap(), random_giphy.unwrap())
}

async fn tenor(
    client: &Client<HttpsConnector<HttpConnector>>,
    q: &String,
    apikey: String,
) -> tenor::types::Search {
    let tenor = tenor::Tenor::new(client, apikey);

    let search_tenor = tenor
        .random(&byte_serialize(q.as_bytes()).collect(), 1)
        .await;

    search_tenor.unwrap()
}

// https://media.tenor.com/images/2bfd030f6db53d738fcc08d3e9a3afbe/tenor.gif (squiddy)
// https://media.tenor.com/images/a53a589ea59868ab7458d4006c080458/tenor.gif boo (monther&co)
// https://media.tenor.com/images/cac3fc4dc1a2f027a8e8aaadc8b4f888/tenor.gif partial frames
