pub mod ws_msg
{


  #[macro_export] 
    macro_rules! ws_error {
        ( $x:expr) => {
            { 
            serde_json::to_string(&WsMessage::error($x.to_string())).unwrap()
            }
    };
}

    use chrono::{DateTime,Local};
    use actix::Message;
    use serde::{Serialize, Deserialize};

    use crate::{device::device::Device, automatisation::voice_recognition::voice_recognition::ScenarioTypes, typedef::typedef::DeviceId};

    #[derive(Serialize, Deserialize, Debug, Message, Clone)]
    #[rtype(result = "Result<(), serde_json::Error>")]
    pub struct WsMessage
    {
        pub message_type : WsMessageType,
        pub payload : String
    }
    impl WsMessage
    {
        pub fn device_update(tgt : &Device) -> Result<WsMessage, serde_json::Error>
        {
            let parse_res = serde_json::to_string(tgt);
            match parse_res{
                Ok(dev_str) => Ok(WsMessage { 
                    message_type: WsMessageType::DEVICE_UPDATE, 
                    payload: dev_str }),
                Err(err) => return Err(err),
            }

        }

        pub fn value(device_id : DeviceId , value: String) -> Result<WsMessage, serde_json::Error>
        {
            let parse_res = serde_json::to_string(&PayloadValue { device_id, value });
            match parse_res{
                Ok(vec_str) => Ok(WsMessage {
                    message_type: WsMessageType::VALUE,
                    payload : vec_str }),
                Err(err) => return Err(err),
                
            }

        }

        pub fn device_list(tgt : Vec<&Device>) -> Result<WsMessage, serde_json::Error>
        {
            let parse_res = serde_json::to_string(&tgt);
            match parse_res{
                Ok(vec_str) => Ok(WsMessage {
                    message_type: WsMessageType::DEVICE_LIST,
                    payload : vec_str }),
                Err(err) => return Err(err),
                
            }

        }

        pub fn scenario_update(scenario_type : ScenarioTypes, scenario_id : Option<usize>, scenario_payload : String, completed: Option<bool>) -> Result<WsMessage, serde_json::Error>
        {
            let parse_res = serde_json::to_string(&PayloadScenarioUpdate { scenario_type, scenario_id, scenario_payload, completed });
            match parse_res{
                Ok(vec_str) => Ok(WsMessage {
                    message_type: WsMessageType::SCENARIO_UPDATE,
                    payload : vec_str }),
                Err(err) => return Err(err),
                
            }

        }
        
        pub fn error(message : String) -> WsMessage
        {
            WsMessage {message_type: WsMessageType::ERROR, payload: message}
        }


    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[allow(non_camel_case_types)]
    pub enum WsMessageType
    {
        //Receive
        VALUE_GET,
        DEVICE_CMD,
        SCENARIO_UPDATE,

        

        //Send
        DEVICE_LIST,
        SCENARIO_LIST,
        DEVICE_UPDATE,
        VALUE,

        UNKNOWN,
        ERROR   
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
    pub struct PayloadValue
    {
        pub device_id : String,
        pub value : String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PayloadGetResponse
    {
        pub device_id : String,
        pub value : Option<String>
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct PayloadScenarioUpdate
    {
        pub scenario_type : ScenarioTypes,
        pub scenario_id : Option<usize>,
        pub completed : Option<bool>,
        pub scenario_payload : String
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PayloadScenarioTimedToggle
    {
        pub sensor_id : String,
        pub time : String //should be ISO-8601
        
    }
   
    #[derive(Serialize, Deserialize, Debug)]
    pub struct PayloadScenarioSensorConditional
    {
        pub sensor_id : String,
        pub treshold : i32,
        pub cmp_over : bool,
        pub target_device: String
    }
    
    pub struct PayloadScenarioRead
    {
        pub device_id : String,
        pub key_to_read : String
    }
}

