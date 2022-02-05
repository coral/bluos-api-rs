use anyhow::Result;
use bluos_api_rs::{BluOS, Discovery};

#[tokio::main]
async fn main() -> Result<()> {
    // Doing it manually
    //let bluos = BluOS::new(Ipv4Addr::from_str("10.0.1.36")?, None).await?;

    // Manually is boring, let's use mDNS to discover this!
    let device = Discovery::discover_one().await?;
    let bluos = BluOS::new_from_discovered(device)?;

    let status = bluos.status().await?;
    dbg!(status);

    let playlist = bluos.queue(None).await?;
    dbg!(playlist);
    Ok(())
}
