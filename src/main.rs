use crate::ascii::gif::frame::AsciiGifFrame;
use crate::ascii::gif::player::AsciiGifPlayer;
use crate::ascii::gif::AsciiGif;
use crate::cli::Cli;
use crate::http::get;
use gif::{ColorOutput, Decoder, SetParameter};
use hyper::client::HttpConnector;
use hyper::Client;
use hyper_tls::HttpsConnector;
use std::env;
use structopt::StructOpt;
use url::form_urlencoded::byte_serialize;

pub mod ascii;
pub mod cli;
pub mod giphy;
pub mod http;
pub mod tenor;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Error loading env variables");
    let giphy_api_key = env::var("GIPHY_API_KEY").expect("Error loading giphy apikey");
    let tenor_api_key = env::var("TENOR_API_KEY").expect("Error loading tenor apikey");

    let (w, h) = term_size::dimensions().unwrap();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let args: Cli = Cli::from_args();
    let url = match (args.q, args.id) {
        (None, None) => None,
        (Some(q), _) => Some(if args.giphy {
            search_giphy(&client, &q, giphy_api_key).await
        } else {
            search_tenor(&client, &q, tenor_api_key).await
        }),
        (_, Some(id)) => Some(if args.giphy {
            id_giphy(&client, &id, giphy_api_key).await
        } else {
            id_tenor(&client, &id, tenor_api_key).await
        }),
    };

    match url {
        Some(url) => {
            let mut decoder = Decoder::new(get(&client, &url).await.unwrap());
            decoder.set(ColorOutput::RGBA);
            let mut decoder = decoder.read_info().unwrap();
            let gif_width = decoder.width().clone();
            let gif_height = decoder.height().clone();

            let mut frames: Vec<AsciiGifFrame> = Vec::new();
            while let Some(frame) = decoder.read_next_frame().unwrap() {
                frames.push(frame.into())
            }

            let ascii_gif = AsciiGif::new(frames, gif_width, gif_height);
            let mut player = AsciiGifPlayer::new(h as u16, w as u16);
            player.play(&ascii_gif, true);
        }
        _ => println!("no gif found :("),
    }
}

async fn id_giphy(
    client: &Client<HttpsConnector<HttpConnector>>,
    id: &String,
    apikey: String,
) -> String {
    giphy::Giphy::new(client, apikey)
        .by_id(&byte_serialize(id.as_bytes()).collect())
        .await
        .expect("no results fodun using giphy")
        .data
        .images
        .fixed_width_small
        .url
        .expect("no url for preview_gif using giphy")
        .clone()
}

async fn id_tenor(
    client: &Client<HttpsConnector<HttpConnector>>,
    id: &String,
    apikey: String,
) -> String {
    unimplemented!();
}

async fn search_giphy(
    client: &Client<HttpsConnector<HttpConnector>>,
    q: &String,
    apikey: String,
) -> String {
    giphy::Giphy::new(client, apikey)
        .random(&byte_serialize(q.as_bytes()).collect())
        .await
        .expect("no results found using giphy")
        .data
        .images
        .fixed_width_small
        .url
        .expect("no url for preview_gif using giphy")
        .clone()
}

async fn search_tenor(
    client: &Client<HttpsConnector<HttpConnector>>,
    q: &String,
    apikey: String,
) -> String {
    tenor::Tenor::new(client, apikey)
        .random(&byte_serialize(q.as_bytes()).collect(), 1)
        .await
        .expect("no results found using tenor")
        .results
        .first()
        .expect("no results found using tenor")
        .media
        .first()
        .expect("no media found using tenor")
        .nanogif
        .url
        .clone()
}
