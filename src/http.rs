use bytes::buf::Reader;
use hyper::body::Buf;
use hyper::client::HttpConnector;
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use std::error::Error;
use std::str::FromStr;

pub async fn get(
    client: &Client<HttpsConnector<HttpConnector>>,
    url: &str,
) -> Result<Reader<impl Buf>, Box<dyn Error>> {
    let response = client.get(Uri::from_str(url)?).await?;
    let body = hyper::body::aggregate(response).await?;
    Ok(body.reader())
}
