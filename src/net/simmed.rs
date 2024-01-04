pub mod simmed
{
    use core::time;
    use std::thread;

    use crate::{device::{device::Device, device::DeviceType}, net::device_update::device_updates::{MQTTUpdate, DeviceUpdateType, MQTTList}};
    use mqtt::{ConnectOptions, MessageBuilder, Properties};
    use paho_mqtt as mqtt;
    use rand::{thread_rng, Rng};
    use serde::Serialize;
    use serde_json::{Map, Value};

    pub fn simulate_devices(device_list: Vec<Device>)
    {
        
        let broker_conn = mqtt::Client::new("tcp://localhost:1883").unwrap();
        let conn_options : ConnectOptions = ConnectOptions::new_v5();
        let token = broker_conn.connect(conn_options.clone()).unwrap();
        
        
        let mut list_props = Properties::new();
        list_props.push_val(mqtt::PropertyCode::PayloadFormatIndicator, 1).expect("failed to add property");
        list_props.push_string(mqtt::PropertyCode::ContentType, "add_device").expect("failed to add property");
        let list_message = MessageBuilder::default()
        .qos(1)
        .topic("add_device").properties(list_props)
        .payload(serde_json::to_vec(&MQTTList::new(device_list.clone())).unwrap())
        .finalize();

        broker_conn.publish(list_message).unwrap();
        let mut dev_props = Properties::new();
        dev_props.push_val(mqtt::PropertyCode::PayloadFormatIndicator, 1).expect("failed to add property");
        dev_props.push_string(mqtt::PropertyCode::ContentType, "device_update").expect("failed to add property");
        
        loop{
            thread::sleep(time::Duration::from_secs(3));
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
                        .topic("device_update")
                        .finalize();

                        match broker_conn.publish(updated_message) {
                            Ok(_) => continue,
                            Err(e) => {
                                match e {
                                    mqtt::Error::PahoDescr(_, _) => reattempt_connection(&broker_conn, conn_options.clone()).unwrap_or_else(|e| 
                                        panic!("Failed to restablish connection! {}", e.to_string())),
                                    _ => panic!("Man MQTT Shit broke: {}", e.to_string())
                                }
                            }
                        }
                        
                    }
                    Err(e) => {
                        println!("MQTTUpdate failed to serialize for device {}! \n {}", device.get_id() , e.to_string());
                        continue;
                    }
                    
                }
            }
        }


    }

    fn reattempt_connection(broker_conn: &mqtt::Client, conn_options: mqtt::ConnectOptions) -> Result<(), paho_mqtt::Error> {
        loop
        {
            match broker_conn.connect(conn_options.clone())
            {
                Ok(_) => return Ok(()),
                Err(e) => return Err(e),
            }
        }
    }



}
