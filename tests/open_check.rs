//! tests/open_check.rs

use open_here::OpenTarget;
use open_here::client;
use open_here::server;
use open_here::setup_logger;

use std::thread;

#[test]
fn server_test() {
    setup_logger(4);

    let target = "http://localhost:1234".to_string();

    let server = server::Server::new(server::Config {
        host: "localhost:0".to_string(),
        dry_run: true, // we do not actually want to open something
    })
    .unwrap();

    let port = server.get_port().unwrap();

    let _server_thread = thread::spawn(|| server.run());

    let res = client::OpenClient::new(format!("http://localhost:{}", port))
        .open(&OpenTarget {
            target: target.clone(),
        })
        .unwrap();

    assert!(res.contains("Would run:") && res.contains(&target));
}
