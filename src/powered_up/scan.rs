use crate::Result;
use futures::stream::StreamExt;
use lego_powered_up::PoweredUp;

pub async fn run() -> Result<()> {
    println!("Scanning Bluetooth for hubs...");
    println!("Press Ctrl+C to stop");

    let mut pu = PoweredUp::init().await?;

    let events = pu.scan().await?;
    tokio::pin!(events);

    while let Some(hub) = events.next().await {
        println!("Discovered hubs: {hub:?}");
    }

    Ok(())
}
