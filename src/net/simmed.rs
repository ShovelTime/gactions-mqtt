pub mod simmed
{
    use core::time;
    use std::{thread, time::Duration};

    use crate::{device::{device::Device, device::DeviceType}, net::device_update::device_updates::{MQTTUpdate, DeviceUpdateType, MQTTList, MQTTStatus}};
    use mqtt::{ConnectOptions, MessageBuilder, Properties, Message};
    use paho_mqtt as mqtt;
    use rand::{thread_rng, Rng};
    use serde::Serialize;
    use serde_json::{Map, Value};

    pub fn simulate_devices(mut device_list: Vec<Device>)
    {
        let mut root_online = false;
        
        let broker_conn = mqtt::Client::new("tcp://localhost:1883").expect("we fucked up the broker innit");
        let conn_options : ConnectOptions = ConnectOptions::new_v5();
        let _token = broker_conn.connect(conn_options.clone()).unwrap_or_else(|_| { // keep trying
            // until we have established a connection;
            let mut res = broker_conn.connect(conn_options.clone());
            while !broker_conn.is_connected()
            {
                thread::sleep(Duration::from_secs(5));
                res = broker_conn.connect(conn_options.clone())
            }
            return res.unwrap();
            
        });
        let _ = broker_conn.subscribe_many(&["device_cmd", "root_online"], &[1, 2]);
        
        let consumer = broker_conn.start_consuming();
        let mut list_props = Properties::new();
        list_props.push_val(mqtt::PropertyCode::PayloadFormatIndicator, 1).expect("failed to add property");
        list_props.push_string(mqtt::PropertyCode::ContentType, "add_device").expect("failed to add property");

        
        let list_message = MessageBuilder::default()
        .qos(1)
        .topic("add_device").properties(list_props)
        .payload(serde_json::to_vec(&MQTTList::new(device_list.clone())).unwrap())
        .finalize();
        

        
        //broker_conn.publish(list_message).unwrap();
        let mut dev_props = Properties::new();
        dev_props.push_val(mqtt::PropertyCode::PayloadFormatIndicator, 1).expect("failed to add property");
        dev_props.push_string(mqtt::PropertyCode::ContentType, "device_update").expect("failed to add property");
        
        loop{
            thread::sleep(time::Duration::from_secs(3));
            while !consumer.is_empty()
            {
                for message in consumer.try_iter().collect::<Vec<Option<Message>>>()

                {
                    match message{
                        Some(msg) => {
                                match msg.properties().get_string(mqtt::PropertyCode::ContentType).unwrap_or("unknown".to_string()).as_str()
                                {
                                    "device_cmd" => 
                                    {
                                        match serde_json::from_str::<MQTTUpdate>(&msg.payload_str()){
                                            Ok(cmd) => {
                                                match cmd.update_type{
                                                    DeviceUpdateType::CONN_CHANGE => todo!(),
                                                    DeviceUpdateType::ACTIVATION_CHANGE => {
                                                        let Some(dev) = device_list.iter_mut().find(|x| {*x == cmd.device_id}) else {continue};
                                                        // if we get None, we dont own the device, so we dont care
                                                        let Some(n_bool) = cmd.update_fields.get("activated") else {continue};
                                                        dev.activated = n_bool.as_bool().unwrap_or(true);
                                                    },
                                                    DeviceUpdateType::VALUE_CHANGE => todo!(),
                                                    DeviceUpdateType::ALL => todo!(),
                                                }    
                                            },
                                            Err(_) => todo!(),
                                        }
                                    },
                                    "root_online" => {
                                        match serde_json::from_str::<MQTTStatus>(&msg.payload_str())
                                        {
                                            Ok(status) => {
                                                        match status.connected
                                                        {
                                                            true => {
                                                                if root_online
                                                                    {continue}
                                                                else
                                                                {
                                                                    root_online = true;
                                                                    let _ = broker_conn.publish(list_message.clone());
                                                                }
                                                            },
                                                            false => {
                                                                if !root_online
                                                                    {continue}
                                                                else
                                                                {
                                                                    root_online = false;
                                                                }
                                                            }, 
                                                        }
                                                    },
                                            Err(_) => println!("Wrong JSON format on root_online payload! \n {:?}", msg),
                                        }
                                            
                                    },
                                    _ => println!("Unknown message received! \n {:?}", msg)
                                }
                            },
                        None => continue,
                    }
                }
                
            }
            
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

                if !root_online {continue};

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

    pub fn reattempt_connection(broker_conn: &mqtt::Client, conn_options: mqtt::ConnectOptions) -> Result<(), paho_mqtt::Error> {
        loop
        {
            match broker_conn.connect(conn_options.clone())
            {
                Ok(_) => return Ok(()),
                Err(e) => return Err(e),
            }
        }
    }
    pub async fn reattempt_connection_async(broker_conn: &mqtt::AsyncClient, conn_options: mqtt::ConnectOptions) -> Result<(), paho_mqtt::Error>
    {
        loop
        {
            match broker_conn.connect(conn_options.clone()).await
            {
                Ok(_) => return Ok(()),
                Err(e) => return Err(e),
            }
        }
    }


}
