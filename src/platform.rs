use std::path::PathBuf;
use udev::Device;
use zbus::dbus_interface;

pub struct CtrlPlatform {
    pub path: PathBuf
}

impl CtrlPlatform {
    pub fn new() -> Result<Self, ()> {
        let mut enumerator = udev::Enumerator::new().unwrap();

        enumerator.match_subsystem("platform").unwrap();
        enumerator.match_sysname("aorus_laptop").unwrap();
        if let Some(device) = (enumerator.scan_devices().map_err(|err| {
            println!("Could not scan devices: {:?}", err)
        })?)
        .next()
        {
            return Ok(Self {
                path: device.syspath().to_path_buf(),
            });
        }
        Err(())
    }
}

#[dbus_interface(name = "com.gigabyte.Platform")]
impl CtrlPlatform {
    // TODO: Implement
    fn switch_fan_mode(&self, value: i32) -> i32 {
        let mut device = Device::from_syspath(&self.path).unwrap();
        device.set_attribute_value("fan_mode", &(value).to_string()).unwrap();
        0
    }
    fn set_fan_speed(&self, value: i32) -> i32 {
        let mut device = Device::from_syspath(&self.path).unwrap();
        device.set_attribute_value("fan_custom_speed", &(value).to_string()).unwrap();
        0
    }
    fn set_charge_mode(&self, value: i32) -> i32 {
        let mut device = Device::from_syspath(&self.path).unwrap();
        device.set_attribute_value("charge_mode", &(value).to_string()).unwrap();
        0
    }
    fn set_charge_limit(&self, value: i32) -> i32 {
        let mut device = Device::from_syspath(&self.path).unwrap();
        device.set_attribute_value("charge_limit", &(value).to_string()).unwrap();
        0
    }
}