pub mod simmed
{
    use core::time;
    use std::thread;

    use crate::{devices::{traits::devices::Device, sensors::sensors::DeviceType}, net::device_update::device_updates::{MQTTUpdate, DeviceUpdateType}};
    use mqtt::{ConnectOptions, MessageBuilder, Properties};
    use paho_mqtt as mqtt;
    use rand::{thread_rng, Rng};
    use serde::Serialize;
    use serde_json::{Map, Value};

    pub fn simulate_devices(device_list: Vec<Device>)
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
        .payload(serde_json::to_vec(&device_list).unwrap())
        .finalize();

        broker_conn.publish(list_message).unwrap();
        let mut dev_props = Properties::new();
        dev_props.push_string(mqtt::PropertyCode::PayloadFormatIndicator, "add_device").unwrap();
        
        loop{
            thread::sleep(time::Duration::from_secs(3));
            let mut value_updates : Vec<MQTTUpdate> = Vec::new();
            for device in &device_list
            {
                let mut rng = thread_rng();
                let update_type = DeviceUpdateType::VALUE_CHANGE;
                let mut new_val: Map<String, Value> = Map::new();
                match device.device_type
                {
                    DeviceType::TempSensor => {
                        new_val.insert("value".to_string(), Some(rng.gen_range(15..25).to_string()).into());
                    },
                    DeviceType::LuxSensor => {
                        new_val.insert("value".to_string(), Some(rng.gen_range(100..400).to_string()).into());
                    },
                    DeviceType::Light => {
                        new_val.insert("value".to_string(), Some(rng.gen_range(0..1).to_string()).into());
                    },

                    //Programmer fucked up lmao. But it doesnt need to crash since this is just mocking.
                    _ => continue
                }

                let updated_device = MQTTUpdate{
                    update_type : update_type,
                    device_id: device.get_id().to_string(),
                    topic: device.topic.clone(),
                    update_fields : new_val
                };

                let update_payload = serde_json::to_vec(&updated_device);
                match update_payload {
                    Ok(msg) => {
                        let updated_message = MessageBuilder::default()
                        .properties(dev_props.clone())
                        .payload(msg)
                        .topic(device.topic.clone())
                        .finalize();
                        broker_conn.publish(updated_message).unwrap();
                    }
                    Err(e) => {
                        println!("MQTTUpdate failed to serialize for device {}! \n {}", device.get_id() , e.to_string());
                        continue;
                    }
                    
                }
            }
        }


    }


}