pub mod room {

    use crate::{devices::traits::devices::Device, typedef::typedef::RoomId};

    pub struct Room {
        room_name: RoomId,
        device_list: Vec<Device>,
    }
}
