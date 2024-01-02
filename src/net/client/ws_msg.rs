pub mod ws_msg
{


    use actix::Message;
    use serde::{Serialize, Deserialize};

    use crate::{device::device::Device, automatisation::voice_recognition::voice_recognition::ScenarioTypes};

    #[derive(Serialize, Deserialize, Debug, Message)]
    #[rtype(result = "Result<(), serde_json::Error>")]
    pub struct WsMessage
    {
        pub message_type : WsMessageType,
        pub payload : String
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[allow(non_camel_case_types)]
    pub enum WsMessageType
    {
        //Receive
        VALUE_GET,
        DEVICE_CMD,

        //Send & Receive
        DEVICE_UPDATE,
        SCENARIO_UPDATE,

        //Send
        DEVICE_LIST,
        VALUE,

        UNKNOWN
    }
    impl Into<String> for WsMessageType
    {
        fn into(self) -> String
        {
            match self
                {
                    WsMessageType::VALUE_GET => "VALUE_GET".to_string(),
                    WsMessageType::DEVICE_CMD => "DEVICE_CMD".to_string(),
                    WsMessageType::DEVICE_UPDATE => "DEVICE_UPDATE".to_string(),
                    WsMessageType::SCENARIO_UPDATE => "SCENARIO_UPDATE".to_string(),
                    WsMessageType::DEVICE_LIST => "DEVICE_LIST".to_string(),
                    WsMessageType::VALUE => "VALUE".to_string(),
                    _ => "UNKNOWN".to_string()
                }
        }
    }
    impl From<String> for WsMessageType
    {
        fn from(str: String) -> WsMessageType 
        {
            match str.as_str()
                {
                    "VALUE_GET" => WsMessageType::VALUE_GET,
                    "DEVICE_CMD" => WsMessageType::DEVICE_CMD,
                    "DEVICE_UPDATE" => WsMessageType::DEVICE_UPDATE,
                    "SCENARIO_UPDATE" => WsMessageType::SCENARIO_UPDATE,
                    "DEVICE_LIST" => WsMessageType::DEVICE_LIST,
                    "VALUE" => WsMessageType::VALUE,
                    _ => WsMessageType::UNKNOWN 
                }
        }
    }


    #[derive(Serialize, Deserialize, Debug)]
    pub enum CommandType
    {
            TOGGLE,
            ENABLE,
            DISABLE,
            UNKNOWN
            
    }
//     impl Into<String> for CommandType in uppercase
    impl Into<String> for CommandType
    {
        fn into(self) -> String
        {
            match self
                {
                    CommandType::TOGGLE => "TOGGLE".to_string(),
                    CommandType::ENABLE => "ENABLE".to_string(),
                    CommandType::DISABLE => "DISABLE".to_string(),
                    _ => "UNKNOWN".to_string()
                }
        }
    }
    // do the same for From<String>
    impl From<String> for CommandType
    {
        fn from(str: String) -> CommandType 
        {
            match str.as_str() 
                {
                    "TOGGLE" => CommandType::TOGGLE,
                    "ENABLE" => CommandType::ENABLE,
                    "DISABLE" => CommandType::DISABLE,
                    _ => CommandType::UNKNOWN 
                }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PayloadDeviceUpdate
    {
        pub device : Device
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PayloadDeviceCommand
    {
        pub device_id : String,
        pub command : CommandType
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PayloadScenarioUpdate
    {
        pub scenario_type : ScenarioTypes,
        //pub scenario : Scenario
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PayloadDeviceList
    {
        pub device_list : Vec<Device>
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PayloadGetValue
    {
        pub topic : String,
        pub device_id : String
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PayloadGetResponse
    {
        pub device_id : String,
        pub value : Option<String>
    }
}
