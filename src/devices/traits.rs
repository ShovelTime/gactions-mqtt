pub mod devices {
    use serde::{Serialize, Deserialize};

    use crate::devices::sensors::sensors::DeviceType;



    #[derive(Serialize, Deserialize, Clone)]
    pub struct Device {

        pub device_type : DeviceType,
        pub name : String,
        pub connected : bool,
        pub activated: bool,
        pub value: String
    }
    impl Device{
        fn get_name(&self) -> String
        {
            self.name.clone()
        }
        fn get_activated(&self) -> bool
        {
            self.activated
        }
        fn get_connection_status(&self) -> bool
        {
            self.connected
        }
        fn get_value(&self) -> Option<String>
        {
            Some(self.value.clone())
        }
    }

// Whether or not we are going to use a standarised type is TBD
}
