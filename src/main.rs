mod crash;
mod vncscan;
mod api;

use warp::{Filter, http::Response, Rejection, Reply};
use std::net::SocketAddr;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() {
    api::api_init(); // Initialize the API (Kinda buggy, but it works)
    let host = "127.0.0.1";
    let port = "9080"; // TODO: Make this public when it will be needed
    crash::main(); // Initialize the crash handler. TODO: Add an option to disable this maybe?
    let static_files = warp::fs::dir("res");

    let log = warp::log::custom(|info| {
        if let Some(remote_addr) = info.remote_addr() {
            println!("[LOG] {} visited 📄{}", remote_addr, info.path());
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

    let scan_route = warp::path("api")
        .and(warp::path("scan"))
        .and(warp::header::<String>("ip"))
        .map(|ip: String| {
            format!("Scanning IP: {}", ip)
        });

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Failed to parse address");

    let routes = static_files.or(not_found).with(log);
    println!("[LOG] 🚀 OpenResolvuim is running at http://{}:{}", host, port);

    warp::serve(routes).run(addr).await;
}