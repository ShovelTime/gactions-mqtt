pub mod devices {
    use std::ops::DivAssign;

    use serde::{Serialize, Deserialize};

    use crate::devices::sensors::sensors::DeviceType;



    #[derive(Serialize, Deserialize, Clone)]
    pub struct Device {

        pub device_id: String,
        pub device_type : DeviceType,
        pub name : String,
        pub connected : bool,
        pub activated: bool,
        pub value: String
    }
    impl Device{
        pub fn new(device_id: String, device_type: DeviceType, name : String) -> Device
        {

            Device { device_id: device_id, device_type: device_type, name: name, connected: true, activated: true, value: String::new() }
        }
        pub fn get_name(&self) -> String
        {
            self.name.clone()
        }
        pub fn get_activated(&self) -> bool
        {
            self.activated
        }
        pub fn get_connection_status(&self) -> bool
        {
            self.connected
        }
        pub fn get_value(&self) -> Option<String>
        {
            Some(self.value.clone())
        }
    }

// Whether or not we are going to use a standarised type is TBD
}
