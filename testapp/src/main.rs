use std::{thread, time::Duration};

use libcommon::prelude::*;

#[timer]
#[logsetup("testlog")]
#[tokio::main]
async fn main() -> Result<()> {
    info!("Hello, world!");
    tokio::spawn(async {
        thread::sleep(Duration::from_secs(2));
        info!("Hello, world! from spawn.");
    })
    .await?;
    Ok(())
}
