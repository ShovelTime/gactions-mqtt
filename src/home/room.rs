pub mod room {

    use crate::devices::traits::devices::Device;

    pub struct Room {
        room_name: String,
        device_list: Vec<Box<dyn Device>>,
    }
}
