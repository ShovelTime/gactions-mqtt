pub mod device {

    use serde::{Serialize, Deserialize};

    use crate::{typedef::typedef::{DeviceId, Topic}, DEVICE_COUNTER};
    use std::{sync::atomic::{AtomicUsize, Ordering}, collections::HashMap};


    pub struct DeviceCounters
    {
            counters : HashMap<DeviceType, AtomicUsize>,
    }
    impl DeviceCounters
    {
        pub fn new() -> DeviceCounters
        {
            DeviceCounters { counters: HashMap::new()}
        }

        pub fn get_num(&mut self, in_type : DeviceType) -> usize
        {
            if !self.counters.keys().any(|x| *x == in_type)
            {
                let _res = self.counters.insert(in_type.clone(), AtomicUsize::new(0));
        
            } 
            return self.counters.get(&in_type).unwrap().fetch_add(1,Ordering::SeqCst);   
        }
    }


    #[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
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
        pub fn new(device_type: DeviceType, _name : String, topic: Topic) -> Device
        {

            
            let name : String;
            match device_type
            {
                DeviceType::TempSensor => { name = "Temp ".to_string(); },

                DeviceType::LuxSensor => { name = "Lux ".to_string(); },
                DeviceType::Light => { name = "Light ".to_string();},
            }

            let device_id = format!( "{} {}", name, DEVICE_COUNTER.lock().unwrap().get_num(device_type));

            Device { device_id, device_type, name, connected: true, activated: true, value: None, topic }
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
