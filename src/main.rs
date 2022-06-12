#[path = "net.rs"]
mod net;

#[tokio::main]
async fn main() {
    net::net_info().await.unwrap();
    println!("Performing 100kB download");
    net::test_download(100_000_000).await.unwrap();
}
