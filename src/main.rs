mod crash;

use warp::{Filter, http::Response};
use std::net::SocketAddr;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() {
    crash::main(); // Initialize the crash handler. TODO: Add an option to disable this maybe?
    let static_files = warp::fs::dir("res");

    let log = warp::log::custom(|info| {
        if let Some(remote_addr) = info.remote_addr() {
            println!("{} - {}", remote_addr, info.path());
        }
    });

    let not_found = warp::any().map(|| {
        let path = Path::new("res/404.html");
        let body = fs::read_to_string(path).unwrap_or_default();

        Response::builder()
            .status(404)
            .header("Content-Type", "text/html")
            .body(body)
            .unwrap()
    });

    let routes = static_files.or(not_found).with(log);

    let addr: SocketAddr = "127.0.0.1:9080".parse().unwrap();
    println!("Serving OpenResolvium on http://{}", addr);

    warp::serve(routes).run(addr).await;
}