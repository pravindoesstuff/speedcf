use futures_util::StreamExt;

const UP_TARGET: &str = "https://speed.cloudflare.com/__up";

lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

pub async fn test_download(target_size: u64) -> Result<(), reqwest::Error> {
    let mut stream = CLIENT
        .get(format!("https://speed.cloudflare.com/__down?bytes={}", target_size))
        .send()
        .await?
        .bytes_stream();
    let pb = indicatif::ProgressBar::new(target_size);

    pb.set_style(indicatif::ProgressStyle::default_bar()
                 .template("[{elapsed_precise:.cyan/blue}] [{bar:40}] {bytes_per_sec} {spinner}")
                 .progress_chars("=> "));

    while let Some(block) = stream.next().await {
        pb.inc(block?.len() as u64)
    }

    pb.finish_with_message("Finished Downloado");
    pb.finish_and_clear();
    Ok(())
}
