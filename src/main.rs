mod platform;

use std::{error::Error, future::pending, env};
use udev::Device;
use zbus::ConnectionBuilder;

use crate::platform::Greeter;


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

    // Get the platform device created by our kernel driver.
    let mut driver: Device;
    let mut enumerator = udev::Enumerator::new().unwrap();

    enumerator.match_subsystem("platform").unwrap();

    for device in enumerator.scan_devices().unwrap() {
        println!("found device using kernel driver {:?}: {:?}", device.sysname(), device.syspath());
        if device.sysname() == "gigabyte_laptop" {
            driver = device.clone();
            for attribute in device.attributes() {
                println!("attribute {:?} = {:?}", attribute.name(), attribute.value());
            }
        }
    }

    // Start zbus server (TODO: Use system connection instead of session)
    let greeter = Greeter { count: 0 };
    let _conn = ConnectionBuilder::session()?
        .name("org.zbus.MyGreeter")?
        .serve_at("/org/zbus/MyGreeter", greeter)?
        .build()
        .await?;

    // Do other things or go to wait forever
    pending::<()>().await;

    Ok(())
}
