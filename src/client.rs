//! open-here client

use crate::cli;
use crate::cmd;

use reqwest::Client;
use tokio::*;


use std::env;
use envconfig::Envconfig;

#[derive(Envconfig)]
struct Config {
    #[envconfig(from = "OPEN_HOST", default = "127.0.0.1:9123")]
    pub host: String,
}


struct OpenClient {
    client: Client,
    server: String,
}

impl OpenClient {
    fn new(server: String) -> Self {
        Self {
            client: Client::new(),
            server,
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
    let cfg = Config::init_from_env().unwrap();
    let server = format!("http://{}", cfg.host);

    let client = OpenClient::new(server);

    match client.open(open).await {
        Ok(_) => {},
        Err(_) => {},
    }
}
