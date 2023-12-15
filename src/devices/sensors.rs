pub mod sensors
{
    use crate::devices::traits::devices::Device;

    use rand::{self, Rng, thread_rng};
    use serde::{Deserialize, Serialize};


    #[derive(Clone, Copy, Serialize, Deserialize)]
    pub enum DeviceType
    {
        TempSensor,
        LuxSensor,
        Light
    }
    /*
    #[derive(Clone)]
    pub struct LuxSensor
    {
        name: String,
        connected: bool,
        value_in: tokio::sync::watch::Receiver<String>

    }
    impl Device for LuxSensor
    {
        fn get_name(&self) -> String {
            self.name.clone()
        }

        fn get_activated(&self) -> bool {
            self.connected
        }

        fn get_connection_status(&self) -> bool {
            self.connected
        }

        fn get_values(&self) -> Option<String> {
            let mut rng = thread_rng();
            Some(rng.gen_range(100..600).to_string())
        }
    }

    #[derive(Clone)]
    pub struct Light
    {

        name : String,
        connected : bool,
        toggled : bool
    }
    impl Device for Light
    {
        fn get_name(&self) -> String {
            self.name.clone()
        }

        fn get_activated(&self) -> bool {
            self.toggled
        }

        fn get_connection_status(&self) -> bool {
            self.connected
        }

        fn get_values(&self ) -> Option<String> {
            Some(self.toggled.to_string())
        }
    }

    #[derive(Clone)]
    pub struct TempSensor
    {

        name : String,
        connected : bool,
        value: String,
        value_in: tokio::sync::watch::Receiver<String>


    }
    impl Device for TempSensor
    {
        fn get_name(&self) -> String {
            self.name.clone()
        }

        fn get_activated(&self) -> bool {
            self.connected
        }

        fn get_connection_status(&self) -> bool {
            self.connected
        }

        fn get_values(&self ) -> Option<String> {
            self.value
        }
    }
    */
}