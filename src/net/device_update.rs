
pub mod device_updates
{
    use serde::{Serialize, Deserialize};
    use serde_json::{Map, Value};

    use crate::{device::{device::Device}, net::client::ws_msg::ws_msg::CommandType};


    #[derive(Serialize, Deserialize, Clone)]
    #[allow(non_camel_case_types)]
    pub enum DeviceUpdateType
    {
        CONN_CHANGE,
        ACTIVATION_CHANGE,
        VALUE_CHANGE,
        ALL
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
    pub struct MQTTStatus
    {
        pub connected : bool
    }// used for topic root_online

    #[derive(Serialize, Deserialize, Clone)]
    pub struct MQTTUpdate
    {
        pub update_type : DeviceUpdateType,
        pub device_id : String,
        pub topic: String,
        pub update_fields : Map<String, Value>
    }

    //Create MQTTCommand struct with derives:
    #[derive(Serialize, Deserialize)]
    pub struct MQTTCommand
    {
        pub device_id : String,
        pub command : CommandType
    }
    
    #[derive(Serialize, Deserialize)]
    pub struct MQTTList
    {
        pub device_list : Option<Vec<Device>>  
    }
    impl MQTTList
    {
        pub fn new(device_list : Vec<Device>) -> Self
        {
            if device_list.is_empty()
            {
                MQTTList{device_list : None}
            }
            else
            {
                MQTTList{device_list : Some(device_list)}
            }
            
        }
    }
}
