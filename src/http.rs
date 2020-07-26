use bytes::buf::ext::Reader;
use bytes::buf::BufExt;
use hyper::body::Buf;
use hyper::client::HttpConnector;
use hyper::{Client, Uri};
use hyper_tls::HttpsConnector;
use std::error::Error;
use std::str::FromStr;

pub async fn get(
    client: &Client<HttpsConnector<HttpConnector>>,
    url: &String,
) -> Result<Reader<impl Buf>, Box<dyn Error>> {
    let response = client.get(Uri::from_str(url.as_str())?).await?;
    let body = hyper::body::aggregate(response).await?;
    Ok(body.reader())
}
