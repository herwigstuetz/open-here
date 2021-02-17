//! open-here client

use crate::cli;
use crate::cmd;

use envconfig::Envconfig;
use reqwest::Client;
use std::fmt;

/// Configuration from the environment for the open-here client
#[derive(Envconfig)]
struct Config {
    /// Host and optional port on which open-here server is listening on
    #[envconfig(from = "OPEN_HOST", default = "127.0.0.1:9123")]
    pub host: String,
}

/// An error that can occur during opening targets
#[derive(Debug)]
pub enum OpenError {
    /// A HTTP error during sending the HTTP request
    HttpError { msg: String },
    ServerError { err: cmd::OpenError },
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
struct OpenClient {
    /// HTTP client
    client: Client,

    /// open-here server host (and port)
    server: String,
}

impl OpenClient {
    /// Instantiate a new `OpenClient`. It keeps an internal HTTP Client
    /// for connection pooling
    fn new(server: String) -> Self {
        Self {
            client: Client::new(),
            server,
        }
    }

    /// Send a request to open `target` on the open-here server
    async fn open(&self, target: cli::OpenTarget) -> Result<()> {
        let url = format!("{}/open", &self.server);
        let req = self
            .client
            .get(&url)
            .query(&[("target", &target.target.to_string())]);

        tracing::debug!("Sent request: {:?}", &req);
        let resp = req.send().await?;

        let res: Option<cmd::OpenError> = resp.json().await?;
        match res {
            None => Ok(()),
            Some(err) => {
                tracing::trace!("{}", err);
                Err(OpenError::ServerError { err })
            }
        }
    }
}

#[tokio::main]
pub async fn open(open: cli::OpenTarget) -> Result<()> {
    let cfg = Config::init_from_env().unwrap();
    let server = format!("http://{}", cfg.host);

    let client = OpenClient::new(server);
    client.open(open).await?;

    Ok(())
}
