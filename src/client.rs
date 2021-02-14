//! open-here client

use crate::cli;
use crate::cmd;

use reqwest::Client;
use tokio::*;

struct OpenClient {
    client: Client,
    server: String,
}

impl OpenClient {
    fn new() -> Self {
        Self {
            client: Client::new(),
            server: std::string::String::from("http://127.0.0.1:8010"),
        }
    }

    async fn open(&self, target: cli::OpenTarget) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}/open", &self.server);
        let req = self.client.get(&url).query(&[("target", &target.target.to_string())]);

        tracing::debug!("Sent request: {:?}", &req);
        let r = req.send().await?;

        Ok(r)
    }
}


#[tokio::main]
pub async fn open(open: cli::OpenTarget) {

    let client = OpenClient::new();

    match client.open(open).await {
        Ok(_) => {},
        Err(_) => {},
    }
}
