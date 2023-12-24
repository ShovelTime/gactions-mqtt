pub mod simmed
{
    use core::time;
    use std::thread;

    use crate::{devices::{traits::devices::Device, sensors::sensors::DeviceType}, net::device_update::device_updates::{MQTTDevice, MQTTUpdate}};
    use mqtt::{ConnectOptions, MessageBuilder, Properties};
    use paho_mqtt as mqtt;
    use rand::{thread_rng, Rng};
    use serde::Serialize;

    pub fn simulate_devices(device_list: Vec<MQTTDevice>)
    {
        
        let broker_conn = mqtt::Client::new("tcp://localhost:1883").unwrap();
        let conn_options : ConnectOptions = ConnectOptions::new_v5();
        let token = broker_conn.connect(conn_options).unwrap();
        
        let mut list_props = Properties::new();
        list_props.push_string(mqtt::PropertyCode::PayloadFormatIndicator, "device_list").unwrap();
        let list_message = MessageBuilder::default()
        .qos(1)
        .retained(true)
        .topic("device_list").properties(list_props)
        .payload(bincode::serialize(&device_list).unwrap())
        .finalize();

        broker_conn.publish(list_message).unwrap();
        let mut dev_props = Properties::new();
        dev_props.push_string(mqtt::PropertyCode::PayloadFormatIndicator, "device_update").unwrap();
        
        loop{
            thread::sleep(time::Duration::from_secs(3));
            let mut value_updates : Vec<MQTTUpdate> = Vec::new();
            for device in &device_list
            {
                let mut rng = thread_rng();
                let new_val: u32;
                match device.device_type
                {
                    DeviceType::TempSensor => {
                        new_val = rng.gen_range(15..25);
                    },
                    DeviceType::LuxSensor => {
                        new_val = rng.gen_range(100..400);
                    },
                    DeviceType::Light => {
                        new_val = 1;
                    },
                }
                let updated_device = MQTTUpdate{
                    device_id: device.device_id.clone(),
                    topic: device.topic.clone(),
                    value: new_val.to_string(),
                };
                let updated_message = MessageBuilder::default()
                .properties(dev_props.clone())
                .payload(bincode::serialize(&updated_device).unwrap())
                .topic(device.topic.clone())
                .finalize();
                broker_conn.publish(updated_message).unwrap();
                
            }
        }


    }


}