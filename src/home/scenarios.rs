pub mod scenarios
{
    use std::{sync::atomic::Ordering, ops::Range, };

    
    use serde_json::{Value, Map};
    use tokio::{time::{sleep_until, Instant, Duration}, sync::mpsc::UnboundedSender};

    use crate::{net::{client::{ws_conn::messaging::send_ws_message, ws_msg::ws_msg::WsMessage}, device_update::device_updates::{MQTTUpdate, DeviceUpdateType}}, typedef::typedef::{DeviceId, ScenarioId}, SCENARIO_COUNTER, SCENARIO_LIST, DEVICE_CONTAINER, automatisation::voice_recognition::voice_recognition::ScenarioTypes};

    pub trait Scenario
    {
        fn get_id(&self) -> usize;
        fn start(&self);
        fn get_type(&self) -> ScenarioTypes;
        

    }
    impl PartialEq<usize> for dyn Scenario
    {
        fn eq(&self, other: &usize) -> bool {
            &self.get_id() == other
        }
    }
    impl PartialEq<dyn Scenario> for dyn Scenario
    {
        fn eq(&self, other: &dyn Scenario) -> bool {
            self.get_id() == other.get_id()
        }
    }
    pub struct TimedToggle
    {
            s_id : usize,
            time_to_trigger: Instant,
            devices: Vec<DeviceId>,
            tx: UnboundedSender<MQTTUpdate>
                        
    }
    impl Scenario for TimedToggle{
        fn get_id(&self) -> usize {
            self.s_id
        }
    
        fn start(&self)
        {
            let time = self.time_to_trigger.clone();
            let devs = self.devices.clone();
            let s_id = self.get_id();
            let tx = self.tx.clone();
            tokio::spawn(async move {
                TimedToggle::start_toggle(time, devs, s_id, tx).await
            });
        }

        fn get_type(&self) -> ScenarioTypes {
            ScenarioTypes::TIMED 
        }


    }
    impl TimedToggle
    {
        pub fn new(time_to_trigger: Instant, devices: Vec<DeviceId>, tx: UnboundedSender<MQTTUpdate> ) -> ScenarioId 
        {
            let mut n_devices = Vec::new();
            for device in devices
            {
                    n_devices.push(device);
            }
            let mut lock = SCENARIO_LIST.write().expect("Why did Scenario List die");
            let id = SCENARIO_COUNTER.fetch_add(1, Ordering::SeqCst);

            lock.push(Box::new(TimedToggle
            {
                s_id : id,
                time_to_trigger,
                devices: n_devices,
                tx
            }));
            id
            }
       pub async fn start_toggle(time_to_trigger : Instant, devices : Vec<DeviceId>, s_id : ScenarioId, tx: UnboundedSender<MQTTUpdate>) -> Result<(), ()>
        {
            
            sleep_until(time_to_trigger).await;
            match DEVICE_CONTAINER.write()
            {
                Ok(mut hash) => {
                    let tgt_devices = hash.values_mut().flatten().filter(|x| {devices.iter().any(|y| {x == y})});//.into_iter(); 
                    for device in tgt_devices
                    {
                        device.toggle();
                        send_ws_message(WsMessage::device_update(device).expect("wow we really failed to parse the device huh"));
                        //TODO: send mqtt update
                        let n_activated = device.activated;
                        let mut map = Map::<String, Value>::new();
                        map.insert("activated".to_string(), n_activated.to_string().into());
                        let update = MQTTUpdate{ update_type: DeviceUpdateType::ACTIVATION_CHANGE, device_id: device.get_id().to_string(), topic: device.topic.clone(), update_fields: map };
                        let _ = tx.send(update);
                        remove_scenario(s_id);

                    }

               },
                Err(err) => panic!("who the fuck poisoned the lock, and why did we not crash yet. \n\n {}", err),
            }
            //send_ws_message(WsMessage::scenario_update(ScenarioTypes::TIMED, Some(s_id), "".to_string(), Some(true)).expect("Finished Scenario Conditional did you not serialize"));
            Ok(())
        }

    }
    
    pub struct ConditionalTrigger
    {
        s_id : usize,
        sensor_id : DeviceId,
        //sensor : Device,
        treshold : Range<i32>,
        tgt_dev : DeviceId,
        tx : UnboundedSender<MQTTUpdate>

        
    }
    impl Scenario for ConditionalTrigger
    {
        fn get_id(&self) -> usize {
            self.s_id
        }
        fn start(&self)
        {
            let treshold = self.treshold.clone();
            let sensor_id = self.sensor_id.clone();
            let tgt_dev = self.tgt_dev.clone();
            let s_id = self.s_id.clone();
            let tx = self.tx.clone();
            tokio::spawn(async move {
                ConditionalTrigger::watch(treshold, sensor_id, tgt_dev, s_id, tx).await;
            });
        }

        fn get_type(&self) -> ScenarioTypes {
            ScenarioTypes::SENSOR_CONDITIONAL
        }
    }
    impl ConditionalTrigger
    {
        pub fn new(sensor_id : DeviceId, treshold : Range<i32>, tgt_dev: DeviceId, tx : UnboundedSender<MQTTUpdate>) -> Result<usize, &'static str>
        {

            //let dev; 
            /*
            {
                let dev_lock = DEVICE_CONTAINER.read().expect("Device list poisoned!");
                let mut dev_list = dev_lock.values().flatten();
                if !dev_list.clone().any(|x| {*x == tgt_dev}) {return Err("target device not found!")}
                let Some(_res) = dev_list.find(|x| { *x == sensor_id}) else {return Err("Sensor not found!")};
                dev = res.clone()
            }
            */
            let mut lock = SCENARIO_LIST.write().expect("Scenario list went byebye");
            let id = SCENARIO_COUNTER.fetch_add(1, Ordering::SeqCst);
            lock.push(Box::new(ConditionalTrigger{
                s_id : id,
                sensor_id,
                //sensor: dev,
                treshold,
                tgt_dev,
                tx
            }));
            Ok(id)
                
        }

        pub async fn watch(treshold: Range<i32>, sensor_id : DeviceId, tgt_dev: DeviceId, s_id : ScenarioId, tx: UnboundedSender<MQTTUpdate>)
        {
            let mut interval = tokio::time::interval(Duration::from_secs(2));
            loop
            {
                interval.tick().await;
                {
                    let r_lock = DEVICE_CONTAINER.read().expect("who panicked with the device lock reeee");
                    let Some(sensor) = r_lock.values().flatten().find(|x| {*x == sensor_id}) else {
                        send_ws_message(WsMessage::error(format!("Sensor {} for Scenario {} no longer exists!", sensor_id, s_id)));
                        break;
                    };
                    let Some(s_value) = sensor.get_value() else {continue};
                    if treshold.contains(&(s_value.parse().unwrap_or(0)))
                    {
                        drop(r_lock);
                        let mut w_lock = DEVICE_CONTAINER.write().expect("damn the lock died in between the drop?");
                        let Some(tgt) = w_lock.values_mut().flatten().find(|x| {*x == tgt_dev}) else {send_ws_message(WsMessage::error(format!("Device {} for Scenario {} no longer exists!", tgt_dev, s_id)));
                        return};
                        tgt.toggle();
                        let n_activated = tgt.activated;
                        let mut map = Map::<String, Value>::new();
                        map.insert("activated".to_string(), n_activated.to_string().into());
                        let update = MQTTUpdate{ update_type: DeviceUpdateType::ACTIVATION_CHANGE, device_id: tgt_dev, topic: tgt.topic.clone(), update_fields: map };
                        let _ = tx.send(update);
                        drop(w_lock);
                        remove_scenario(s_id);
                        return;
                    }
                }
            }
        }
    }

    pub fn remove_scenario(s_id : ScenarioId)
    {
        {
            let mut lock = SCENARIO_LIST.write().expect("Why is the scenario list ded");
            {
                for i in 0..lock.len()
                {
                    if lock[i].get_id() == s_id
                    {
                        let scenario = lock.remove(i);
                        send_ws_message(WsMessage::scenario_update(scenario.get_type(), Some(s_id), "".to_string(), Some(true)).expect("Finished Scenario did you serialize"));
                        return;

                    }
                }

            }
        }

    }
}
