pub mod device {

    use serde::{Serialize, Deserialize};

    use crate::typedef::typedef::{DeviceId, Topic};



    #[derive(Clone, Copy, Serialize, Deserialize, Debug)]
    pub enum DeviceType
    {
        TempSensor,
        LuxSensor,
        Light
    }

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
        pub fn toggle(&mut self)
        {
            match self.activated
                {
                    true => self.activated = false,
                    false => self.activated = true,
                }
        }
        pub fn get_connection_status(&self) -> bool
        {
            self.connected
        }
        pub fn get_value(&self) -> Option<String>
        {
            match &self.value {
                Some(val) => Some(val.clone()),
                None => None
            }
        }
        pub fn set_value(&mut self, value: Option<String>)
        {
            self.value = value;
        }
        pub fn update(&mut self, new_dev: &Device)
        {
            if new_dev == self.device_id
            {
                self.device_type = new_dev.device_type;
                self.name = new_dev.name.clone();
                self.connected = new_dev.connected;
                self.activated = new_dev.activated;
                self.value = new_dev.value.clone();
            }             
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

    impl PartialEq<String> for &Device
    {
        fn eq(&self, other: &String) -> bool
        {
            self.device_id == *other
        }
    }

    impl PartialEq<String> for &mut Device
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
    
    impl PartialEq<&mut Device> for Device
    {
        fn eq(&self, other: &&mut Device) -> bool {
            self.device_id == *other.device_id
        }
    }
}
