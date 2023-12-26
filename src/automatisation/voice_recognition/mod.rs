
pub mod voice_recognition
{
    use serde::{Serialize, Deserialize};



    #[allow(non_camel_case_types)]
    #[derive(Serialize, Deserialize, Debug)]
    pub enum ScenarioTypes
    {
        TIMED,
        SENSOR_CONDITIONAL,
        READ_SENSOR_OR_STATE,
        GENERAL_KENOBI
    }

    

    

}