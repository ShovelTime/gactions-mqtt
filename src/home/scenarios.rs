pub mod scenarios
{
    use std::{collections::HashMap, sync::{RwLock, Arc}};

    use actix::WeakAddr;
    use tokio::time::{sleep_until, Instant, Duration};

    use crate::{device::device::Device, net::client::{ws_conn::messaging::{WsConn, send_ws_message, send_ws_message_async}, ws_msg::ws_msg::{WsMessage, WsMessageType}}, typedef::typedef::DeviceId};

    pub struct TimedToggle
    {
            time_to_trigger: Instant,
            devices: Vec<DeviceId>,
            device_hash: Arc<RwLock<HashMap<String, Vec<Device>>>>,
            conn_list: Arc<RwLock<Vec<WeakAddr<WsConn>>>>
            
    }
    impl TimedToggle
    {
        pub fn new(time_to_trigger: Instant, devices: Vec<DeviceId>, device_hash: Arc<RwLock<HashMap<String, Vec<Device>>>>, conn_list: Arc<RwLock<Vec<WeakAddr<WsConn>>>>   ) -> TimedToggle
        {
            let mut n_devices = Vec::new();
            for device in devices
            {
                    n_devices.push(device);
            }
            TimedToggle
            {
                time_to_trigger,
                devices: n_devices,
                device_hash,
                conn_list
            }
        }
       pub async fn start_toggle(&mut self) -> Result<(), ()>
        {
            
            sleep_until(self.time_to_trigger).await;
            match self.device_hash.write()
            {
                //This has terrible time complexity, should be rewritten if possible.
                Ok(mut hash) => {
                    let tgt_devices = hash.values_mut().flatten().filter(|x| {self.devices.iter().any(|y| {x == y})});//.into_iter(); 
                    for device in tgt_devices
                    {
                        device.toggle();
                        send_ws_message_async(Arc::clone(&self.conn_list), WsMessage::device_update(device).expect("wow we really failed to parse the device huh")).await;
                        //send mqtt update
                    }

               },
                Err(err) => panic!("who the fuck poisoned the lock, and why did we not crash yet. \n\n {}", err),
            }
            Ok(())
        }

    }
}
