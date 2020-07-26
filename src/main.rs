use crate::ascii_frame::AsciiFrame;
use crate::http::get;
use gif::SetParameter;
use hyper::client::HttpConnector;
use hyper::Client;
use hyper_tls::HttpsConnector;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub mod ascii_frame;
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

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let (_, _) = giphy(&client, q, giphy_api_key).await;
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
    dbg!(&url);

    let mut decoder = gif::Decoder::new(get(&client, &url).await.unwrap());
    decoder.set(gif::ColorOutput::RGBA);
    let mut decoder = decoder.read_info().unwrap();
    let frame = decoder.read_next_frame().unwrap().unwrap();

    let ascii_frame: AsciiFrame = frame.into();

    let ascii_frame_as_string: String = ascii_frame.to_string();
    let mut file = File::create(Path::new("./frame.txt")).unwrap();
    file.write(ascii_frame_as_string.as_bytes()).unwrap();

    println!("{}", ascii_frame_as_string);
}

async fn giphy(
    client: &Client<HttpsConnector<HttpConnector>>,
    q: &String,
    apikey: String,
) -> (giphy::types::Search, giphy::types::Random) {
    let giphy = giphy::Giphy::new(client, apikey);
    let search_giphy = giphy.search(q, 1).await;
    dbg!(&search_giphy);

    let random_giphy = giphy.random(q).await;
    dbg!(&random_giphy);

    (search_giphy.unwrap(), random_giphy.unwrap())
}

async fn tenor(
    client: &Client<HttpsConnector<HttpConnector>>,
    q: &String,
    apikey: String,
) -> tenor::types::Search {
    let tenor = tenor::Tenor::new(client, apikey);
    let search_tenor = tenor.random(q, 1).await;
    dbg!(&search_tenor);

    search_tenor.unwrap()
}

// https://media.tenor.com/images/2bfd030f6db53d738fcc08d3e9a3afbe/tenor.gif (squiddy)
// https://media.tenor.com/images/a53a589ea59868ab7458d4006c080458/tenor.gif boo (monther&co)
