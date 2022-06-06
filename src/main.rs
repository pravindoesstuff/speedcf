#[path = "net.rs"]
mod net;

#[tokio::main]
async fn main() {
    net::test_download(1000000000).await.unwrap();
}
