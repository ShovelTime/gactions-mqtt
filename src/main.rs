use device::device::Device;
use net::{simmed::simmed::simulate_devices, device_update::device_updates::{MQTTUpdate, MQTTList}};
use tokio::{net::TcpListener, sync::broadcast::*};
use std::{sync::{RwLock, Arc}, any::Any};
use paho_mqtt::{Message, Client, ConnectOptions, AsyncClient};
use crate::{device::device::DeviceType, net::device_update::device_updates::DeviceUpdateType};
use std::{thread, collections::HashMap};
use actix_web::{App, HttpServer, web::Data};

pub mod automatisation;
pub mod home;
pub mod net;
pub mod device;
pub mod typedef;

//TODO: Fix many of the simple unwrap()s.
#[deny(clippy::unwrap_used)]
#[tokio::main(worker_threads = 8)]
async fn main() {

    let device_container : Arc<RwLock<HashMap<String, Vec<Device>>>> = Arc::new(RwLock::new(HashMap::new::<>()));
    
    {
        let mut device_hash = device_container.write().expect("Failed to lock device container!, this should never happen");
        let device1 = Device::new("temp1".to_string(), DeviceType::TempSensor, "Temperature 1".to_string(), "temperature".to_string());
        device_hash.insert("temperature".to_string(), Vec::new());
        device_hash.get_mut("temperature").expect("wait litterally how did we panic here").push(device1)
    }

    let sim_thread =thread::spawn(move || // Simulated Devices
    {
        let device = Device::new(
                "Temperature sensor1".to_string(),
                DeviceType::TempSensor,
                "".to_string(),
                "temp1".to_string());
        simulate_devices(vec!(device));
    });

    /*
    let mqtt_receiver: Client = Client::new("tcp://localhost:1883").unwrap();
    let lock_recv = Arc::clone(&device_container);
    let recv_thread = thread::spawn(move || {
        handle_message(mqtt_receiver, lock_recv);
    });
    */
    let a_client = AsyncClient::new("tcp://localhost:1883").unwrap();
    let dev_box: Box<dyn Any + Send + Sync> = Box::new(Arc::clone(&device_container));
    let _ = a_client.user_data().insert(&dev_box);
    let conn_options : ConnectOptions = ConnectOptions::new_v5();
    a_client.set_message_callback(handle_message_async);
    let _conn_token = a_client.connect(conn_options).await.expect("Failed to connect to MQTT Broker");

    let listener = TcpListener::bind("").await.expect("Failed to bind Socket"); 
    HttpServer::new(move || { 
        App::new()
        .app_data(Data::new(Arc::clone(&device_container)))
    })
    .bind("0.0.0.0:18337").expect("Failed to start Websocket Listener!")
    .run()
    .await.expect("Failed to start HTTP Server");



}


fn handle_message_async(client: &AsyncClient, recv_message : Option<Message>)
{

    let device_lock = client.user_data().as_ref().unwrap().downcast_ref::<Arc<RwLock<HashMap<String, Vec<Device>>>>>().unwrap();
    let Some(msg) = recv_message else {
        println!("Received empty message!");
        return;
    };
    let msg_type = msg.properties().get_string(paho_mqtt::PropertyCode::PayloadFormatIndicator).unwrap_or("unknown".to_string());
                
    match msg_type.as_str()
    {

        "device_update" => {
            let mqtt_update  = serde_json::from_slice(&msg.payload());
            match mqtt_update
            {
                Ok(dev_list) => update_device(dev_list, &device_lock),

                Err(err) =>
                {
                    println!("Error Parsing Message! Printing payload: \n\n {} \n\n {}", err.to_string() ,std::str::from_utf8(&msg.payload()).unwrap_or("PARSE FAILED"));
                }
            }
        }
        "add_device" =>
        {
            let device_list = serde_json::from_slice(&msg.payload());
            match device_list
            {
                Ok(list) => add_to_device_list(list, &device_lock),
                Err(err) => println!("Unrecognized MQTT message received! Printing payload: \n\n {} \n\n {}", err.to_string() ,std::str::from_utf8(&msg.payload()).unwrap_or("PARSE FAILED")),
            }
        }

        _ => println!("Error Parsing Message! Printing payload: \n\n {}", std::str::from_utf8(&msg.payload()).unwrap_or("PARSE FAILED"))
    }
            
} 


fn handle_message(client: Client, device_lock: Arc<RwLock<HashMap<String, Vec<Device>>>>)
{

    let conn_options : ConnectOptions = ConnectOptions::new_v5();
    let _server_res = client.connect(conn_options).expect("Failed to connect to MQTT Broker");
    let msg_recv = client.start_consuming();
    client.subscribe_many(&["temperature", "add_device", "remove_device"], &[1, 0]).expect("Failed to subscribe to topics");
 

    while client.is_connected()
    {

        match msg_recv.recv()
        {
            Ok(x) =>
            {
                let Some(msg) = x else {
                    continue;
                };
                let msg_type = msg.properties().get_string(paho_mqtt::PropertyCode::PayloadFormatIndicator).unwrap_or("unknown".to_string());
                
                match msg_type.as_str()
                {

                    "device_update" => {
                        let mqtt_update  = serde_json::from_slice(&msg.payload());
                        match mqtt_update
                        {
                            Ok(dev_list) => update_device(dev_list, &device_lock),

                            Err(err) =>
                            {
                                println!("Error Parsing Message! Printing payload: \n\n {} \n\n {}", err.to_string() ,std::str::from_utf8(&msg.payload()).unwrap_or("PARSE FAILED"));
                            }
                        }
                    }
                    "add_device" =>
                    {
                        let device_list = serde_json::from_slice(&msg.payload());
                        match device_list
                        {
                            Ok(list) => add_to_device_list(list, &device_lock),
                            Err(err) => println!("Unrecognized MQTT message received! Printing payload: \n\n {} \n\n {}", err.to_string() ,std::str::from_utf8(&msg.payload()).unwrap_or("PARSE FAILED")),
                        }
                    }

                    _ => println!("Error Parsing Message! Printing payload: \n\n {}", std::str::from_utf8(&msg.payload()).unwrap_or("PARSE FAILED"))
                }
            
            } 
            Err(err) => panic!("Lost connection to MQTT Broker! \n\n {}", err.to_string()) // TODO: Find a way to recover from this.
    }

    }
}


fn add_to_device_list(m_list : MQTTList,  list_lock : &RwLock<HashMap<String, Vec<Device>>>)
{

    let Some(devices) = m_list.device_list else {
        println!("a device list update was received, but no devices were provided!");
        return
    };
    devices.iter().for_each(|x| 
    {
        match list_lock.write(){
            Ok(mut list) => 
            {
                if list.contains_key(&x.topic)
                {
                    match list.get_mut(&x.topic).unwrap().iter_mut().find(|y| y == &&x)
                    {
                        
                        Some(device) => {
                            println!("Device {} already exists in list, old entry will be overwritten; Old Data: {:?} \n ", x.get_id(), device);
                            device.name = x.name.clone();
                            device.set_value(x.get_value().clone());
                            device.connected = x.connected;
                            device.activated = x.activated;
                            device.device_type = x.device_type;

                        },
                        None => list.get_mut(&x.topic).unwrap().push(x.clone())
                    }
                }
                else
                {
                    list.insert(x.topic.clone(), Vec::new()).unwrap().push(x.clone());
                }
            },

            Err(e) => panic!("RwLock poisoned! {}", e.to_string())
        
        }
    });

}

fn update_device(update: MQTTUpdate, list_lock : &RwLock<HashMap<String, Vec<Device>>>)
{
     match list_lock.write(){

        Ok(mut list) => 
        {
            match list.get_mut(&update.topic)
            {
                Some(dev_list) =>
                {
                    match dev_list.iter_mut().find(|x| **x == update.device_id)
                    {
                        Some(device) => {
                            match update.update_type
                            {
                                DeviceUpdateType::CONN_CHANGE => {
                                    let Some(value) = update.update_fields.get("connected") else {
                                        println!("A connection update was received, but no connected field was provided! Printing payload: \n\n {}", serde_json::to_string(&update).unwrap_or("PARSE FAILED".to_string()));
                                        return;
                                    };
                                    let Some(connected) = value.as_bool() else {
                                        println!("value at connected is not a boolean! Printing payload: \n\n {}", serde_json::to_string(&update).unwrap_or("PARSE FAILED".to_string()));
                                        return;
                                    };

                                    device.connected = connected;
                                }
                                DeviceUpdateType::ACTIVATION_CHANGE => {
                                    let Some(value) = update.update_fields.get("connected") else {
                                        println!("A activation update was received, but no connected field was provided! Printing payload: \n\n {}", serde_json::to_string(&update).unwrap_or("PARSE FAILED".to_string()));
                                        return;
                                    };
                                    let Some(activated) = value.as_bool() else {
                                        println!("value at activated is not a boolean! Printing payload: \n\n {}", serde_json::to_string(&update).unwrap_or("PARSE FAILED".to_string()));
                                        return;
                                    };

                                    device.activated = activated;
                                },
                                DeviceUpdateType::VALUE_CHANGE => {
                                    let Some(value) = update.update_fields.get("value") else
                                    {
                                        println!("value field was empty! Printing payload: \n\n {}", serde_json::to_string(&update).unwrap_or("PARSE FAILED".to_string()));
                                        return;
                                    };
                                    device.set_value(Some(value.to_string()))
                                },
                                _ => {
                                    println!("Unrecognized MQTTUpdate type received! Printing payload: \n\n {}", serde_json::to_string(&update).unwrap_or("PARSE FAILED".to_string()));
                                    return;
                                }
                            }
                        }
                        
                        None => println!("Device {} not found in list! Printing payload: \n\n {}", update.device_id, serde_json::to_string(&update).unwrap_or("PARSE FAILED".to_string()))
                    }
                        
                    }
                    None =>
                    {
                        println!("No Devices found under topic {}. Printing payload: \n\n {}", update.topic , serde_json::to_string(&update).unwrap_or("PARSE FAILED".to_string()));
                    }
                }

            }
            Err(e) => panic!("RwLock poisoned! {}", e.to_string())
        }
    

    }
     
