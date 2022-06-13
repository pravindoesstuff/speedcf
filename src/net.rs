use futures_util::StreamExt;

const UP_TARGET: &str = "https://speed.cloudflare.com/__up";

lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}

pub async fn test_download(target_size: u64) -> Result<std::time::Duration, reqwest::Error> {
    let mut stream = CLIENT
        .get(format!(
            "https://speed.cloudflare.com/__down?bytes={}",
            target_size
        ))
        .send()
        .await?
        .bytes_stream();
    let pb = indicatif::ProgressBar::new(target_size);

    pb.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise:.cyan/blue}] [{bar:40}] {bytes_per_sec} {spinner}")
            .progress_chars("=> "),
    );

    let timer = std::time::Instant::now();

    while let Some(block) = stream.next().await {
        pb.inc(block?.len() as u64)
    }

    let duration = timer.elapsed();

    pb.finish_with_message("Finished Download");
    pb.finish_and_clear();
    Ok(duration)
}

pub async fn net_info() -> Result<(), reqwest::Error> {
    let client_info: serde_json::Value = CLIENT
        .get("https://speed.cloudflare.com/meta")
        .send()
        .await?
        .json()
        .await?;
    let servers: serde_json::Value = CLIENT
        .get("https://speed.cloudflare.com/locations")
        .send()
        .await?
        .json()
        .await?;

    let ip_address = client_info.get("clientIp").unwrap().as_str().unwrap();

    if ip_address.contains(':') {
        println!("Connected via IPv6")
    } else {
        println!("Connected via IPv4")
    }

    println!(
        "Your network: {} (AS{})",
        client_info.get("asOrganization").unwrap().as_str().unwrap(),
        client_info.get("asn").unwrap()
    );

    println!("Your IP address: {}", ip_address);

    Ok(())
}
