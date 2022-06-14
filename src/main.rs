use std::io::Write;

#[path = "net.rs"]
mod net;

#[tokio::main]
async fn main() {
    net::net_info().await.unwrap();
    println!();
    
    if let Err(e) = net::test_download(102400).await {
        panic!("{}", e);
    }
    if let Err(e) = net::test_download(1048576).await {
        panic!("{}", e);
    }
    if let Err(e) = net::test_download(10485760).await {
        panic!("{}", e);
    }
    if let Err(e) = net::test_download(26214400).await {
        panic!("{}", e);
    } 
    if let Err(e) = net::test_download(104857600).await {
        panic!("{}", e);
    }
}
