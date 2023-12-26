pub mod devices {

    use serde::{Serialize, Deserialize};

    use crate::{devices::sensors::sensors::DeviceType, typedef::typedef::{DeviceId, Topic}};

    #[derive(Serialize, Deserialize, Clone, Debug)]
    pub struct Device {
        device_id: DeviceId,
        pub device_type: DeviceType,
        pub topic: Topic,
        pub name: String,
        pub connected: bool,
        pub activated: bool,
        value: Option<String>,
    }
    impl Device{
        pub fn new(device_id: String, device_type: DeviceType, name : String, topic: Topic) -> Device
        {

            Device { device_id: device_id, device_type: device_type, name: name, connected: true, activated: true, value: None, topic: topic }
        }
        pub fn get_id(&self) -> &String
        {
            &self.device_id
        }
        pub fn get_name(&self) -> &String
        {
            &self.name
        }
        pub fn get_activated(&self) -> bool
        {
            self.activated
        }
        pub fn get_connection_status(&self) -> bool
        {
            self.connected
        }
        pub fn get_value(&self) -> &Option<String>
        {
            return &self.value
        }
        pub fn set_value(&mut self, value: Option<String>)
        {
            self.value = value;
        }
    }

    impl Eq for Device {}

    // When it comes to comparing devices, we only care about the device_id, as it is the only thing that is guaranteed to be unique identifier to a device.
    impl PartialEq<String> for Device
    {
        fn eq(&self, other: &String) -> bool
        {
            self.device_id == *other
        }
    }

    impl PartialEq<Device> for Device
    {
        fn eq(&self, other: &Device) -> bool
        {
            self.device_id == *other.device_id
        }
    }

    impl PartialEq<&Device> for Device
    {
        fn eq(&self, other: &&Device) -> bool
        {
            self.device_id == *other.device_id
        }
    }

}
