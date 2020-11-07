use crate::giphy::types::{Random, Search};
use crate::http::get;
use hyper::client::HttpConnector;
use hyper::Client;
use hyper_tls::HttpsConnector;

pub mod types;

const BASE_URL: &str = "https://api.giphy.com/v1/gifs";

pub struct Giphy<'a> {
    http_client: &'a Client<HttpsConnector<HttpConnector>>,
    apikey: String,
}

impl<'a> Giphy<'a> {
    pub fn new(http_client: &'a Client<HttpsConnector<HttpConnector>>, apikey: String) -> Self {
        Giphy {
            http_client,
            apikey,
        }
    }

    pub async fn random(&self, tag: &str) -> Result<Random, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/random?api_key={}&tag={}&rating=g",
            BASE_URL, self.apikey, tag
        );

        let random = serde_json::from_reader(get(&self.http_client, &url).await?)?;
        Ok(random)
    }

    pub async fn search(
        &self,
        search: &str,
        limit: i32,
    ) -> Result<Search, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/search?api_key={}&q={}&limit={}&offset=0&rating=g&lang=en",
            BASE_URL, self.apikey, search, limit
        );

        let search = serde_json::from_reader(get(&self.http_client, &url).await?)?;
        Ok(search)
    }

    pub async fn by_id(&self, id: &str) -> Result<Random, Box<dyn std::error::Error>> {
        let url = format!("{}/{}?api_key={}", BASE_URL, id, self.apikey);

        let by_id = serde_json::from_reader(get(&self.http_client, &url).await?)?;
        Ok(by_id)
    }
}
