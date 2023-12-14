mod sensors
{
    use crate::devices::traits::devices::Device;

    use rand::{self, Rng, thread_rng};
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

    pub struct TempSensor
    {

        name : String,
        connected : bool,
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
            let mut rng = rand::thread_rng();
            Some(rng.gen_range(15..26).to_string())
        }
    }
}