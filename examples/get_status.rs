use anyhow::Result;
use bluos_api_rs::BluOS;

#[tokio::main]
async fn main() -> Result<()> {
    let n = BluOS::new().await?;
    let status = n.get_status().await;
    dbg!(status);
    Ok(())
}
