//! open-here client

use crate::cmd;
use crate::{OpenTarget, Response};

use std::fmt;

use envconfig::Envconfig;
use reqwest::Client;
use structopt::StructOpt;

use bytes::Bytes;

/// Configuration from the environment for the open-here client
#[derive(Debug, StructOpt, Envconfig)]
pub struct Config {
    /// Host and optional port on which open-here server is listening on
    #[envconfig(from = "OPEN_HOST", default = "127.0.0.1:9123")]
    #[structopt(default_value = "127.0.0.1:9123")]
    pub host: String,
}

/// An error that can occur during opening targets
#[derive(Debug)]
pub enum OpenError {
    /// A HTTP error during sending the HTTP request
    HttpError {
        msg: String,
    },
    ServerError {
        err: cmd::OpenError,
    },
}

impl fmt::Display for OpenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpenError::HttpError { msg } => write!(f, "Open failed with HTTP error: {}", msg),
            OpenError::ServerError { err } => write!(f, "Open failed with server error: {}", err),
        }
    }
}

impl From<reqwest::Error> for OpenError {
    fn from(request: reqwest::Error) -> Self {
        OpenError::HttpError {
            msg: request.to_string(),
        }
    }
}

type Result<T> = std::result::Result<T, OpenError>;

/// Client that connects to open-here server and sends "open" requests
pub struct OpenClient {
    /// HTTP client
    client: Client,

    /// open-here server host (and port)
    server: String,
}

impl OpenClient {
    /// Instantiate a new `OpenClient`. It keeps an internal HTTP Client
    /// for connection pooling
    pub fn new(server: String) -> Self {
        Self {
            client: Client::new(),
            server,
        }
    }

    /// Send a request to open `open` on the open-here server
    #[tokio::main]
    pub async fn open(&self, open: &OpenTarget) -> Result<String> {

        let req = match open {
            OpenTarget::Url(target) => {
                let url = format!("{}/open/url", &self.server);
                let req = self.client.get(&url).json(&target);

                req
            }
            OpenTarget::Path(target) => {
                let url = format!("{}/open/path", &self.server);
                let bytes = Bytes::copy_from_slice(target.content.as_slice());

                let req = self
                    .client
                    .get(&url)
                    .query(&target)
                    .body(bytes);

                req
            }
        };

        tracing::debug!("Sent request: {:?}", &req);
        let resp = req.send().await?;

        if resp.status().is_success() {
            let res: Response = resp.json().await?;

            if let Err(err) = &res {
                tracing::trace!("{}", err);
            }

            res.map_err(|err| OpenError::ServerError { err })
        } else {
            Err(OpenError::HttpError { msg: resp.status().to_string() })
        }
    }
}
