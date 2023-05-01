mod platform;

use std::{error::Error, future::pending, env};
use zbus::Connection;

use crate::platform::CtrlPlatform;


// Although we use `async-std` here, you can use any async runtime of choice.
#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let is_service = match env::var_os("IS_SERVICE") {
        Some(val) => val == "1",
        None => false,
    };

    if !is_service {
        println!("gigabyted should only be run as a systemd service!");
        println!("If you need logs, use journalctl -b -u gigabyted");
        println!("gigabyted will now exit");
        return Ok(());
    }

    let connection = Connection::system().await?;

    // Setup interface for kernel driver
    match CtrlPlatform::new() {
        Ok(ctrl) => {
            connection.object_server().at("/com/gigabyte/Platform", ctrl).await.ok();
        }
        Err(err) => {
            println!("CtrlPlatform: {:?}", err);
        }
    }

    connection.request_name("com.gigabyte.daemon").await?;

    // Do other things or go to wait forever
    pending::<()>().await;

    Ok(())
}
