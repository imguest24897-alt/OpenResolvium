use reqwest::Error;

pub async fn scan(ip: &str) {
    match reqwest::get(ip).await {
        Ok(response) => {
            if response.status().is_success() {
                println!("Server is up!");
                screenshot(ip).await;
            } else {
                eprintln!("Server is down :(");
            }
        }
        Err(err) => {
            println!("Failed to reach the server: {}", err);
        }
    }
}
pub async fn screenshot(data: &str) {
    println!("Sadly, screenshot is not implemented.\nIP/URL: {}", data)
}