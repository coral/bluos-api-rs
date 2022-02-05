use anyhow::Result;
use bluos_api_rs::{BluOS, Discovery};

#[tokio::main]
async fn main() -> Result<()> {
    // Doing it manually
    //let bluos = BluOS::new(Ipv4Addr::from_str("10.0.1.36")?, None).await?;

    // Manually is boring, let's use mDNS to discover this!
    // Find the first device in our network
    let device = Discovery::discover_one().await?;

    // Create a new BluOS device from the discovered address
    let bluos = BluOS::new_from_discovered(device)?;

    // Print the status
    let status = bluos.status().await?;
    dbg!(status);

    // List items in the play queue
    let playlist = bluos.queue(None).await?;
    for n in playlist.entries {
        println!("{}", n.title.unwrap_or_default());
    }

    // Resume playback
    bluos.play().await?;

    Ok(())
}
