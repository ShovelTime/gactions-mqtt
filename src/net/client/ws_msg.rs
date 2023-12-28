pub mod ws_msg
{
    use serde::{Serialize, Deserialize};

    use crate::{device::device::Device, automatisation::voice_recognition::voice_recognition::ScenarioTypes};

    #[derive(Serialize, Deserialize, Debug)]
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
        VALUE_GET, //Get value of Device
        DEVICE_CMD,

        //Send & Receive
        DEVICE_UPDATE,
        SCENARIO_UPDATE,

        //Send
        DEVICE_LIST,
        VALUE_RESPONSE
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum CommandType
        {
            TOGGLE,
            ENABLE,
            DISABLE,
            
        }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct PayloadDeviceUpdate
    {
        pub device : Device
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
