use crate::http::get;
use crate::tenor::types::Search;
use hyper::client::HttpConnector;
use hyper::Client;
use hyper_tls::HttpsConnector;
use std::error::Error;

pub mod types;

const BASE_URL: &str = "https://api.tenor.com/v1";

pub struct Tenor<'a> {
    http_client: &'a Client<HttpsConnector<HttpConnector>>,
    apikey: String,
}

impl<'a> Tenor<'a> {
    pub fn new(http_client: &'a Client<HttpsConnector<HttpConnector>>, apikey: String) -> Self {
        Tenor {
            http_client,
            apikey,
        }
    }

    pub async fn search(&self, search: String, limit: i32) -> Result<Search, Box<dyn Error>> {
        let url = format!(
            "{}/search?q={}&key={}&limit={}",
            BASE_URL, search, self.apikey, limit
        );

        let search = serde_json::from_reader(get(&self.http_client, &url).await?)?;
        Ok(search)
    }

    pub async fn random(&self, q: &str, limit: i32) -> Result<Search, Box<dyn Error>> {
        let url = format!(
            "{}/random?q={}&key={}&limit={}",
            BASE_URL, q, self.apikey, limit
        );

        let search = serde_json::from_reader(get(&self.http_client, &url).await?)?;
        Ok(search)
    }

    pub async fn by_id(&self, id: &str) -> Result<Search, Box<dyn std::error::Error>> {
        let url = format!("{}/gifs?ids={}&key={}", BASE_URL, id, self.apikey);

        let by_id = serde_json::from_reader(get(&self.http_client, &url).await?)?;
        Ok(by_id)
    }
}
