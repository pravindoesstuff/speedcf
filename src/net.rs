use futures_util::StreamExt;

const UP_TARGET: &str = "https://speed.cloudflare.com/__up";

lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

pub async fn test_download(target_size: usize) -> Result<(), reqwest::Error> {
    let mut stream = CLIENT
        .get(format!("https://speed.cloudflare.com/__down?bytes={}", target_size))
        .send()
        .await?
        .bytes_stream();
    let mut downloaded_size: usize = 0;

    while let Some(block) = stream.next().await {
        let chunk = block?;
        downloaded_size += chunk.len();
    }
    Ok(())
}
