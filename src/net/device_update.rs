
pub mod device_updates
{
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};

    use crate::device::{device::Device, device::DeviceType};


    #[derive(Serialize, Deserialize)]
    #[allow(non_camel_case_types)]
    pub enum DeviceUpdateType
    {
        CONN_CHANGE,
        ACTIVATION_CHANGE,
        VALUE_CHANGE
    }

    /*
    #[derive(Serialize, Deserialize)]
    pub struct MQTTDevice
    {

        pub device_id : String,
        pub device_type : DeviceType,
        pub connection_status : bool,
        pub activated : bool,
        pub topic : String
        
    }
    */

    #[derive(Serialize, Deserialize)]
    pub struct MQTTUpdate
    {
        pub update_type : DeviceUpdateType,
        pub device_id : String,
        pub topic: String,
        pub update_fields : Map<String, Value>
    }

    #[derive(Serialize, Deserialize)]
    pub struct MQTTList
    {
        pub device_list : Option<Vec<Device>>  
    }
}
