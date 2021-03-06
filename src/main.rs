use crate::cli::{Cli, CliError};
use crate::http::get;
use hyper::client::HttpConnector;
use hyper::Client;
use hyper_tls::HttpsConnector;
use std::cmp::min;
use structopt::StructOpt;
use url::form_urlencoded::byte_serialize;

pub mod ascii;
pub mod cli;
pub mod giphy;
pub mod http;
pub mod postprocessor;
pub mod tenor;

const TENOR_API_KEY: &str = "F491OZRFEBGM";
const GIPHY_API_KEY: &str = "UoCwwfxnPpX4mxC2y4nYEFmTq1hmdoGN";

#[tokio::main]
async fn main() {
    let (w, h) = term_size::dimensions().unwrap();

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let args: Cli = Cli::from_args();

    let url = if args.giphy {
        giphy(args.q, args.id, &client, GIPHY_API_KEY.to_string()).await
    } else {
        tenor(args.q, args.id, &client, TENOR_API_KEY.to_string()).await
    }
    .expect("Error retrieving gif");

    let gif = yaged::decoder::decode(
        get(&client, &url).await.unwrap(),
        yaged::decoder::ColorOutput::RGBA,
    )
    .expect("error decoding gif");

    let gif_width = gif.screen_descriptor().width();
    let gif_height = gif.screen_descriptor().height();

    let encoding = &args.encoding;
    let ascii_frames: Vec<ascii::gif::frame::Frame> = gif
        .frames()
        .iter()
        .map(|frame| (frame, encoding).into())
        .collect();

    let ascii_gif = ascii::gif::Gif::new(ascii_frames, gif_width, gif_height);

    let mut player = ascii::gif::player::Player::new(
        min(h as u16, gif_height),
        min(w as u16, gif_width),
        vec![Box::new(postprocessor::downscaling::Downscaling::new(
            w as u16, h as u16,
        ))],
    );
    player.play(ascii_gif, true);
}

async fn giphy(
    q: Option<String>,
    id: Option<String>,
    client: &Client<HttpsConnector<HttpConnector>>,
    apikey: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let giphy = giphy::Giphy::new(client, apikey);

    match (q, id) {
        (Some(q), _) => Ok(giphy
            .random(&byte_serialize(q.as_bytes()).collect::<String>())
            .await
            .expect("no results found using giphy")
            .data
            .images
            .fixed_width_small
            .url
            .expect("no url for preview_gif using giphy")),
        (_, Some(id)) => Ok(giphy
            .by_id(&byte_serialize(id.as_bytes()).collect::<String>())
            .await
            .expect("no results fodun using giphy")
            .data
            .images
            .fixed_width_small
            .url
            .expect("no url for preview_gif using giphy")),
        (None, None) => Err(Box::new(CliError::WrongParameters)),
    }
}

async fn tenor(
    q: Option<String>,
    id: Option<String>,
    client: &Client<HttpsConnector<HttpConnector>>,
    apikey: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let tenor = tenor::Tenor::new(client, apikey);

    match (q, id) {
        (Some(q), _) => Ok(tenor
            .random(&byte_serialize(q.as_bytes()).collect::<String>(), 1)
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
            .clone()),
        (_, Some(id)) => Ok(tenor
            .by_id(&byte_serialize(id.as_bytes()).collect::<String>())
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
            .clone()),
        (None, None) => Err(Box::new(CliError::WrongParameters)),
    }
}
