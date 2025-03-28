use warp::{Filter, http::Response};
use std::net::SocketAddr;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() {
    let static_files = warp::fs::dir("res");

    let not_found = warp::any().map(|| {
        let path = Path::new("res/404.html");
        let body = fs::read_to_string(path).unwrap_or_default();

        Response::builder()
            .status(404)
            .header("Content-Type", "text/html")
            .body(body)
            .unwrap()
    });

    let routes = static_files.or(not_found);
    
    let addr: SocketAddr = "127.0.0.1:9080".parse().unwrap();
    println!("Serving OpenResolvium on http://{}", addr);
    
    warp::serve(routes).run(addr).await;
}