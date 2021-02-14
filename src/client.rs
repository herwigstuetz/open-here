//! open-here client

use crate::cli;
use crate::cmd;

use reqwest::Client;
use tokio::*;


use std::env;
use envconfig::Envconfig;

#[derive(Envconfig)]
struct Config {
    #[envconfig(from = "OH_HOST")]
    pub oh_host: String,

    #[envconfig(from = "OH_PORT")]
    pub oh_port: Option<u16>,
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
    let cfg = Config::init().unwrap();
    let server = format!("http://{}:{}", cfg.oh_host, cfg.oh_port.unwrap_or(8080));

    let client = OpenClient::new(server);

    match client.open(open).await {
        Ok(_) => {},
        Err(_) => {},
    }
}
