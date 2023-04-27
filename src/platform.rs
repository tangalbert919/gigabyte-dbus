use std::path::PathBuf;
use zbus::{dbus_interface, SignalContext, fdo};

pub struct Greeter {
    pub count: u64
}

#[dbus_interface(name = "org.zbus.MyGreeter1")]
impl Greeter {
    // Can be `async` as well.
    fn say_hello(&mut self, name: &str) -> String {
        self.count += 1;
        format!("Hello {}! I have been called {} times.", name, self.count)
    }

    // Rude!
    async fn go_away(
        &self,
        #[zbus(signal_context)]
        ctxt: SignalContext<'_>,
    ) -> fdo::Result<()> {
        //Self::greeted_everyone(&ctxt).await?;
        //self.done.notify(1);

        Ok(())
    }
}

pub struct CtrlPlatform {
    pub device: PathBuf
}

impl CtrlPlatform {
    // TODO: Implement
    pub fn new() -> Result<Self, ()> {
        let mut enumerator = udev::Enumerator::new().unwrap();

        enumerator.match_subsystem("platform").unwrap();
        enumerator.match_sysname("gigabyte_laptop").unwrap();
        if let Some(device) = (enumerator.scan_devices().map_err(|err| {
            println!("Could not scan devices")
        })?)
        .next()
        {
            return Ok(Self {
                device: device.syspath().to_path_buf(),
            });
        }
        Err(())
    }
}

#[dbus_interface(name = "com.gigabyte.Platform")]
impl CtrlPlatform {
    // TODO: Implement
    fn switch_fan_mode(&self, value: i32) -> i32 {
        2 * 3 * value
    }
}