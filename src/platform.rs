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
    fn set_fan_curve_index(&self, value: i32) -> i32 {
        let mut device = Device::from_syspath(&self.path).unwrap();
        device.set_attribute_value("fan_curve_index", &(value).to_string()).unwrap();
        0
    }
    fn set_fan_curve_data(&self, value: i32) -> i32 {
        let mut device = Device::from_syspath(&self.path).unwrap();
        device.set_attribute_value("fan_curve_data", &(value).to_string()).unwrap();
        0
    }
    // get methods
    fn get_fan_mode(&self) -> i32 {
        let device = Device::from_syspath(&self.path).unwrap();
        let fan_mode = device.attribute_value("fan_mode").unwrap();
        fan_mode.to_str().unwrap().parse::<i32>().unwrap()
    }
    fn get_fan_speed(&self) -> i32 {
        let device = Device::from_syspath(&self.path).unwrap();
        let fan_speed = device.attribute_value("fan_custom_speed").unwrap();
        fan_speed.to_str().unwrap().parse::<i32>().unwrap()
    }
    fn get_charge_mode(&self) -> i32 {
        let device = Device::from_syspath(&self.path).unwrap();
        let charge_mode = device.attribute_value("charge_mode").unwrap();
        charge_mode.to_str().unwrap().parse::<i32>().unwrap()
    }
    fn get_charge_limit(&self) -> i32 {
        let device = Device::from_syspath(&self.path).unwrap();
        let charge_limit = device.attribute_value("charge_limit").unwrap();
        charge_limit.to_str().unwrap().parse::<i32>().unwrap()
    }
    fn get_fan_curve_index(&self) -> i32 {
        let device = Device::from_syspath(&self.path).unwrap();
        let fan_curve_index = device.attribute_value("fan_curve_index").unwrap();
        fan_curve_index.to_str().unwrap().parse::<i32>().unwrap()
    }
    fn get_fan_curve_data(&self) -> (i32, i32) {
        let device = Device::from_syspath(&self.path).unwrap();
        let fan_curve_data = device.attribute_value("fan_curve_data").unwrap();
        let mut parts = fan_curve_data.to_str().unwrap().split_whitespace();
        let temp = parts.next().unwrap().parse::<i32>().unwrap();
        let speed = parts.next().unwrap().parse::<i32>().unwrap();
        (temp, speed)
    }
    fn get_battery_cycle(&self) -> i32 {
        let device = Device::from_syspath(&self.path).unwrap();
        let battery_cycle = device.attribute_value("battery_cycle").unwrap();
        battery_cycle.to_str().unwrap().parse::<i32>().unwrap()
    }
    fn get_usb_s3_toggle(&self) -> i32 {
        let device = Device::from_syspath(&self.path).unwrap();
        let usb_s3_toggle = device.attribute_value("usb_charge_s3_toggle").unwrap();
        usb_s3_toggle.to_str().unwrap().parse::<i32>().unwrap()
    }
    fn get_usb_s4_toggle(&self) -> i32 {
        let device = Device::from_syspath(&self.path).unwrap();
        let usb_s4_toggle = device.attribute_value("usb_charge_s4_toggle").unwrap();
        usb_s4_toggle.to_str().unwrap().parse::<i32>().unwrap()
    }
    fn get_gpu_boost(&self) -> i32 {
        let device = Device::from_syspath(&self.path).unwrap();
        let gpu_boost = device.attribute_value("gpu_boost").unwrap();
        gpu_boost.to_str().unwrap().parse::<i32>().unwrap()
    }
}