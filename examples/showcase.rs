#[cfg(not(feature = "discover"))]
fn main() {
    println!("This example needs the discover feature to be enabled");
}

#[cfg(feature = "discover")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use anyhow::Context;
    use bluos_api_rs::{BluOS, Discovery};

    // Doing it manually
    //let bluos = BluOS::new(Ipv4Addr::from_str("10.0.1.36")?, None).await?;

    // Manually is boring, let's use mDNS to discover this!
    // Find the first device in our network
    let device = Discovery::discover_one().await.context("discover_one")?;

    // Create a new BluOS device from the discovered address
    let bluos = BluOS::new_from_discovered(device).context("new_from_discovered")?;
    println!("discovered: {bluos:?}");

    // Print the status
    let status = bluos.status().await.context("status")?;
    dbg!(status);

    // List items in the play queue
    println!("Queue:");
    let playlist = bluos.queue(None).await.context("queue")?;
    for n in playlist.entries {
        println!("{}", n.title.unwrap_or_default());
    }
    println!();

    println!("Browse:");
    let browse = bluos.browse(None).await.context("browse")?;
    for item in browse.items {
        println!("{} ({:?})", item.text.unwrap_or_default(), item.item_type);
        if item.browse_key.is_some() {
            for item in bluos.browse(item.browse_key.as_deref()).await?.items {
                println!("  {}", item.text.unwrap_or_default());
            }
        }
    }

    // Resume playback
    // bluos.play().await?;

    Ok(())
}
