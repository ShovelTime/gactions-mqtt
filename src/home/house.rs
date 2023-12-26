use std::collections::HashMap;

use crate::typedef::typedef::RoomId;

use super::room::room::Room;


pub struct House{
    
    rooms : HashMap<RoomId , Room>

}