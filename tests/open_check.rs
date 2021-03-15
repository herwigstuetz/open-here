//! tests/open_check.rs

use open_here::client;
use open_here::server;
use open_here::setup_logger;
use open_here::{OpenTarget, UrlTarget, PathTarget};

use std::sync::Once;
use std::thread;

static INIT: Once = Once::new();

pub fn initialize() {
    INIT.call_once(|| {
        // initialization code here
        setup_logger(4);
    });
}

#[test]
fn test_open_url() {
    initialize();

    let target = "http://localhost:1234".to_string();

    let server = server::Server::new(server::Config {
        host: "localhost:0".to_string(),
        dry_run: true, // we do not actually want to open something
        max_filesize: 26214400,
    })
    .unwrap();

    let port = server.get_port().unwrap();

    let _server_thread = thread::spawn(|| server.run());

    let res = client::OpenClient::new(format!("http://localhost:{}", port))
        .open(&OpenTarget::Url(UrlTarget {
            target: target.clone(),
        }))
        .unwrap();

    assert!(res.contains("Would run:") && res.contains(&target));
}

#[test]
fn test_open_file() {
    initialize();

    let target = "image.png".to_string();

    let server = server::Server::new(server::Config {
        host: "localhost:0".to_string(),
        dry_run: true, // we do not actually want to open something
        max_filesize: 26214400,
    })
    .unwrap();

    let port = server.get_port().unwrap();

    let _server_thread = thread::spawn(|| server.run());

    let res = client::OpenClient::new(format!("http://localhost:{}", port))
        .open(&OpenTarget::Path(PathTarget {
            filename: target.clone(),
            content: vec![],
        }))
        .unwrap();

    assert!(res.contains("Would save") && res.contains(&target));
}
