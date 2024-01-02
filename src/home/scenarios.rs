pub mod scenarios
{
    use std::{collections::HashMap, sync::{RwLock, Arc}};

    use tokio::time::{sleep_until, Instant, Duration};

    use crate::device::device::Device;

    pub struct TimedToggle
    {
            time_to_trigger: Instant,
            devices: Vec<Device>,
            device_hash: Arc<RwLock<HashMap<String, Device>>>

    }
    impl TimedToggle
    {
        fn new(time_to_trigger: Instant, devices: Vec<Device>, device_hash: Arc<RwLock<HashMap<String, Device>>>) -> TimedToggle
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
                device_hash
            }
        }
       pub async fn start_toggle(trigger_at : Instant, affected_devices : Vec<Device>) -> Result<(), ()>
        {
            
            sleep_until(trigger_at).await;
            let  
            Ok(())
        }

        async fn trigger(&self)
        {
        }
    }
}
