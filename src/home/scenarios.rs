pub mod scenarios
{
    use std::{collections::HashMap, sync::{RwLock, Arc}};

    use actix::WeakAddr;
    use tokio::time::{sleep_until, Instant, Duration};

    use crate::{device::device::Device, net::client::ws_conn::messaging::WsConn};

    pub struct TimedToggle
    {
            time_to_trigger: Instant,
            devices: Vec<Device>,
            device_hash: Arc<RwLock<HashMap<String, Vec<Device>>>>,
            conn_list: Arc<RwLock<Vec<WeakAddr<WsConn>>>>
            
    }
    impl TimedToggle
    {
        fn new(time_to_trigger: Instant, devices: Vec<Device>, device_hash: Arc<RwLock<HashMap<String, Vec<Device>>>>, conn_list: Arc<RwLock<Vec<WeakAddr<WsConn>>>>   ) -> TimedToggle
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
                Ok(mut hash) => {
                            for device_vec in hash.values_mut()
                            {
                                for device in &mut *device_vec

                                {
                                    if self.devices.contains(device)
                                    {
                                        let Some(tgt_dev) = self.devices.iter_mut().find(|d| **d == device) else {panic!("Reality has broken down, or you just fucked up                                                                                                        the device comparison here, device 1 :\n {:?} \n\n device_vec : {:?} \n 
                                        " , device, self.devices);}; 

                                        device.update(tgt_dev)

                                        
                                    }
                                }
                            }
                    },
                Err(err) => panic!("who the fuck poisoned the lock, and why did we not crash yet. \n\n {}", err),
            }
            Ok(())
        }

        async fn trigger(&self)
        {
        }
    }
}
