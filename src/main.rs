use actix::WeakAddr;
use device::device::Device;
use home::scenarios::scenarios::Scenario;
use net::{simmed::simmed::{simulate_devices, reattempt_connection, reattempt_connection_async}, device_update::device_updates::{MQTTUpdate, MQTTList, MQTTStatus}, client::{ws_conn::messaging::{WsConn, send_ws_message, ws_conn_request}, ws_msg::ws_msg::WsMessage}};
use once_cell::sync::Lazy;
use tokio::{net::TcpListener, sync::{broadcast::{*, self}, mpsc::{UnboundedSender, unbounded_channel, UnboundedReceiver}}};
use std::{sync::{RwLock, Arc, atomic::AtomicUsize}, any::Any, time::Duration};
use paho_mqtt::{Message, Client, ConnectOptions, AsyncClient, ConnectOptionsBuilder, MessageBuilder, Properties, PropertyCode, Error};
use crate::{device::device::DeviceType, net::device_update::device_updates::DeviceUpdateType};
use std::{thread, collections::HashMap};
use actix_web::{App, HttpServer, web::{Data, self}};

pub mod automatisation;
pub mod home;
 #[macro_use]
pub mod net;
pub mod device;
pub mod typedef;

pub static DEVICE_CONTAINER : Lazy<Arc<RwLock<HashMap<String, Vec<Device>>>>> = Lazy::new(|| {Arc::new(RwLock::new(HashMap::new::<>()))});

pub static CONN_LIST : Lazy<Arc<RwLock<Vec<WeakAddr<WsConn>>>>> = Lazy::new(|| {Arc::new(RwLock::new(Vec::new()))});

pub static SCENARIO_COUNTER : AtomicUsize = AtomicUsize::new(0); //yes this will eventually
//crash in a few hundred years, too bad!

pub static SCENARIO_LIST : Lazy<Arc<RwLock<Vec<Box<dyn Scenario + Sync + Send>>>>> = Lazy::new(|| {Arc::new(RwLock::new(Vec::new()))});

pub const MQTT_SENDER : Lazy<Option<UnboundedSender<MQTTUpdate>>> = Lazy::new(|| {None}); 

#[deny(clippy::unwrap_used)]
#[tokio::main(worker_threads = 8)]
async fn main() {



    let device_container : Arc<RwLock<HashMap<String, Vec<Device>>>> = Arc::new(RwLock::new(HashMap::new::<>()));
    let conn_list : Arc<RwLock<Vec<WeakAddr<WsConn>>>> = Arc::new(RwLock::new(Vec::new()));
    let (tx, rx) = unbounded_channel::<MQTTUpdate>();
    MQTT_SENDER.insert(tx.clone());
    
    {
        let mut device_hash = device_container.write().expect("Failed to lock device container!, this should never happen");
        let device1 = Device::new("temp1".to_string(), DeviceType::TempSensor, "Temperature 1".to_string(), "temperature".to_string());
        device_hash.insert("temperature".to_string(), Vec::new());
        device_hash.get_mut("temperature").expect("wait litterally how did we panic here").push(device1)
    }

    let sim_thread =thread::spawn(move || // Simulated Devices
    {
        let temp_device = Device::new(
                "Temperature sensor1".to_string(),
                DeviceType::TempSensor,
                "Temperature Sensor".to_string(),
                "temp1".to_string());
        let lux_device = Device::new(
                "Lux Sensor1".to_string(),
                DeviceType::LuxSensor,
                "Lux Sensor".to_string(),
                "lux1".to_string());
        let lamp_device = Device::new(
                "Lamp1".to_string(),
                DeviceType::Light,
                "Lamp".to_string(),
                "lamp1".to_string());

        simulate_devices(vec!(temp_device, lux_device, lamp_device));
    });

    /*
    let mqtt_receiver: Client = Client::new("tcp://localhost:1883").unwrap();
    let lock_recv = Arc::clone(&device_container);
    let recv_thread = thread::spawn(move || {
        handle_message(mqtt_receiver, lock_recv);
    });
    */



    let mut props = Properties::new();
    props.push_val(PropertyCode::PayloadFormatIndicator, 1).expect("failed to add property");
    props.push_string(PropertyCode::ContentType, "root_online").expect("failed to add property");
    let a_client = AsyncClient::new("tcp://localhost:1883").unwrap();
    //let _ = a_client.user_data().insert(&dev_box);
    let conn_options : ConnectOptions = ConnectOptionsBuilder::new_v5().will_message(
        MessageBuilder::default() // if we lost connection, let em know
            .qos(2)
            .properties(props.clone())
            .retained(true)
            .topic("root_online")
            .payload(serde_json::to_vec(&MQTTStatus{connected: false}).expect("MQTTStatus has fucked serialization"))
            .finalize()
    ).finalize();
    a_client.set_message_callback(handle_message_async);
    a_client.set_connected_callback(|a_cli| {a_cli.subscribe_many(&["add_device", "device_update"], &[2, 1]);

        let mut props = Properties::new();
        props.push_val(PropertyCode::PayloadFormatIndicator, 1).expect("failed to add property");
        props.push_string(PropertyCode::ContentType, "root_online").expect("failed to add property");
        let online = MessageBuilder::default() // advertise root as online
            .qos(2)
            .properties(props)
            .retained(true)
            .topic("root_online")
            .payload(serde_json::to_vec(&MQTTStatus{connected: true}).expect("MQTTStatus has fucked serialization"))
            .finalize();
        a_cli.publish(online);        
    });
    println!("connecting");
    let conn_token = a_client.connect(conn_options.clone()).await;
    println!("conn finished");
    let mqtt_arc = Arc::new(a_client);
    let opt_arc = Arc::new(conn_options);

    match conn_token {
        Ok(_) => (),
        Err(_) => {
            let ref_cli = Arc::clone(&mqtt_arc);
            let reconn_opt_arc = Arc::clone(&opt_arc);
            tokio::spawn(async move {
                while !ref_cli.is_connected()
                {
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    ref_cli.connect(reconn_opt_arc.as_ref().clone());
                }
            });
        },

        }
    
    

    //let listener = TcpListener::bind("").await.expect("Failed to bind Socket"); 


    let conn_opts = Arc::clone(&opt_arc);
    tokio::spawn(async move{
        
        let mut recv = rx;
        
        let mut dev_props = Properties::new();
        dev_props.push_val(PropertyCode::PayloadFormatIndicator, 1).expect("failed to add property");
        dev_props.push_string(PropertyCode::ContentType, "device_update").expect("failed to add property");
        loop
        {
            let msg = recv.recv().await.expect("This sould not crash, otherwise it means that the channel closed on its creation");

            let update_payload = serde_json::to_vec(&msg);
            match update_payload {
                Ok(msg) => {
                    let updated_message = MessageBuilder::default()
                    .properties(dev_props.clone())
                    .payload(msg)
                    .topic("device_update")
                    .finalize();

                    match mqtt_arc.publish(updated_message).await {
                        Ok(_) => continue,
                        Err(e) => {
                            match e {
                                Error::PahoDescr(_, _) => reattempt_connection_async(&mqtt_arc, conn_opts.as_ref().clone()).await.unwrap_or_else(|e| 
                                    panic!("Failed to restablish connection! {}", e.to_string())),
                                _ => panic!("Man MQTT Shit broke: {}", e.to_string())
                            }
                        }
                    }
                        
                }
                Err(e) => {
                    println!("MQTTUpdate failed to serialize! \n {}", e.to_string());
                    continue;
                }
                    
            }
            
        }
    });


    HttpServer::new(move || { 
        App::new().route("/ws", web::get().to(ws_conn_request))
    })
    .bind("0.0.0.0:18337").expect("Failed to start Websocket Listener!")
    .run()
    .await.expect("Failed to start HTTP Server");
    let _ = sim_thread.join();



}

fn handle_message_async(_client: &AsyncClient, recv_message : Option<Message>)
{


    //let device_lock = client.user_data().expect("What do you mean we have nothing in the user_data").downcast_ref::<Arc<RwLock<HashMap<String, Vec<Device>>>>>().expect("This should never crash, ye fucked up somewhere lad");
    let Some(msg) = recv_message else {
        println!("Received empty message!");
        return;
    };
    let msg_type = msg.properties().get_string(paho_mqtt::PropertyCode::ContentType).unwrap_or("unknown".to_string());
    println!("we are so in lads");
                
    match msg_type.as_str()
    {

        "device_update" => {
            let mqtt_update  = serde_json::from_slice(&msg.payload());
            match mqtt_update
            {
                Ok(dev_list) => {update_device(dev_list); println!("Recieved Device Update")},

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
                Ok(list) => add_to_device_list(list),
                Err(err) => println!("Unrecognized MQTT message received! Printing payload: \n\n {} \n\n {}", err.to_string() ,std::str::from_utf8(&msg.payload()).unwrap_or("PARSE FAILED")),
            }
        }

        _ =>  println!("Error Parsing Message! message_type: {} Printing payload: \n\n {}", msg_type, std::str::from_utf8(&msg.payload()).unwrap_or("PARSE FAILED"))  }
            
} 

#[allow(dead_code)]
fn handle_message(client: Client, _device_lock: Arc<RwLock<HashMap<String, Vec<Device>>>>)
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
                            Ok(dev_list) => update_device(dev_list),

                            Err(err) =>
                            {
                                println!("Error Parsing Update Message! Printing payload: \n\n {} \n\n {}", err.to_string() ,std::str::from_utf8(&msg.payload()).unwrap_or("PARSE FAILED"));
                            }
                        }
                    }
                    "add_device" =>
                    {
                        let device_list = serde_json::from_slice(&msg.payload());
                        match device_list
                        {
                            Ok(list) => add_to_device_list(list),
                            Err(err) => println!("Unrecognized MQTT message received! Printing payload: \n\n {} \n\n {}", err.to_string() ,std::str::from_utf8(&msg.payload()).unwrap_or("PARSE FAILED")),
                        }
                    }

                    _ => println!("Error Parsing Message! message_type: {} Printing payload: \n\n {}", msg_type, std::str::from_utf8(&msg.payload()).unwrap_or("PARSE FAILED"))
                }
            
            } 
            Err(err) => panic!("Lost connection to MQTT Broker! \n\n {}", err.to_string()) // TODO: Find a way to recover from this.
    }

    }
}


fn add_to_device_list(m_list : MQTTList)
{

    let Some(devices) = m_list.device_list else {
        println!("a device list update was received, but no devices were provided!");
        return
    };
    devices.iter().for_each(|x| 
    {
        match DEVICE_CONTAINER.write(){
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
                    let vec = vec!(x.clone());
                    list.insert(x.topic.clone(), vec);
                }
                
                send_ws_message( WsMessage::device_list(list.values().flatten().collect()).expect("Damn list didnt properly serialize"));

            },

            Err(e) => panic!("RwLock poisoned! {}", e.to_string())
        
        }
        println!("Added Device");
    });

}

fn update_device(update: MQTTUpdate)
{
     match DEVICE_CONTAINER.write(){

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
                                    device.set_value(Some(value.to_string()));
                                },
                                _ => {
                                    println!("Unrecognized MQTTUpdate type received! Printing payload: \n\n {}", serde_json::to_string(&update).unwrap_or("PARSE FAILED".to_string()));
                                    return;
                                }
                            }
                            send_ws_message(WsMessage::device_update(device).expect("failed to parse Device"));                

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
     
