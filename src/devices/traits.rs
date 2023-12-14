pub mod devices {

    pub trait Device {

        fn get_name(&self) -> String;
        fn get_activated(&self) -> bool;
        fn get_connection_status(&self) -> bool;
        fn get_values(&self ) -> Option<String>;
    }

// Whether or not we are going to use a standarised type is TBD
}
