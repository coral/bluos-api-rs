use anyhow::Result;
use bluos_api_rs::BluOS;

#[tokio::main]
async fn main() -> Result<()> {
    let n = BluOS::new().await?;

    let status = n.status().await?;
    dbg!(status);

    dbg!(n.play().await?);
    Ok(())
}
