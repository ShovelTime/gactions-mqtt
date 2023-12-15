
pub mod device_updates
{
    use serde::{Serialize, Deserialize};

    use crate::devices::{traits::devices::Device, sensors::sensors::DeviceType};

    #[derive(Serialize, Deserialize)]
    pub struct MQTTDevice
    {

        pub device_id : String,
        pub device_type : DeviceType,
        pub topic : String
        
    }

    #[derive(Serialize, Deserialize)]
    pub struct MQTTUpdate
    {
        pub device_id : String,
        pub topic: String,
        pub value: String
    }

    #[derive(Serialize, Deserialize)]
    pub struct MQTTList
    {
        device_list : Option<Vec<MQTTDevice>>
        
    }
}
