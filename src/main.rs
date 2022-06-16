use std::io::Write;

#[path = "net.rs"]
mod net;

#[tokio::main]
async fn main() {
    if let Err(e) = net::net_info().await {
        panic!("{}", e);
    }
    if let Err(e) = download_test().await {
        panic!("{}", e);
    } 
}

async fn download_test() -> Result<(), reqwest::Error> {
    net::test_download(102400).await?;
    net::test_download(1048576).await?;
    net::test_download(10485760).await?;
    net::test_download(26214400).await?;
    net::test_download(104857600).await?;
    Ok(())
}
