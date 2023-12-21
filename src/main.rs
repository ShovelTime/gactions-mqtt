use actix_web::{HttpServer, App};
use devices::traits::devices::Device;
use net::{simmed::simmed::simulate_devices, device_update::device_updates::{MQTTDevice, MQTTUpdate, MQTTList}};
use tokio::{net::TcpListener};
use std::{sync::{RwLock, Mutex}, any::Any};
use paho_mqtt::{AsyncClient, Message, async_client, UserData, Client, ConnectOptions};
use crate::devices::sensors::sensors::DeviceType;
use std::{thread, collections::HashMap};

pub mod automatisation;
pub mod home;
pub mod net;
pub mod devices;

//TODO: Fix many of the simple unwrap()s.

#[tokio::main(worker_threads = 8)]
async fn main() {

    let mut device_container : RwLock<HashMap<String, Vec<Device>>> = RwLock::new(HashMap::new::<>());

    {
        let mut device_hash = device_container.get_mut().unwrap();
        let mut device1 = Device::new("temp1".to_string(), DeviceType::TempSensor, "Temperature 1".to_string());
        device_hash.insert("temperature".to_string(), Vec::new());
        device_hash.get_mut("temperature").unwrap().push(device1)
    }


    
    let mut hash_mutex: Box<dyn Any + Send + Sync + 'static> = Box::new(device_container);

;


    let thread =thread::spawn(move || // Simulated Devices
    {


        let device = MQTTDevice{
                device_id: "Temperature sensor1".to_string(),
                device_type: DeviceType::TempSensor,
                topic: "temp1".to_string()
                
        };
        simulate_devices(vec!(device));
    });


    let mqtt_receiver = AsyncClient::new("tcp://localhost:1883").unwrap();



    thread.join().unwrap();

    

    let listener = TcpListener::bind("0.0.0.0:80").await.expect("Failed to bind Socket"); // replace port with something else;



}

fn handle_message(client: Client, msg: RwLock<HashMap<String, Vec<Device>>>)
{

    let conn_options : ConnectOptions = ConnectOptions::new_v5();
    let server_res = client.connect(conn_options).unwrap();
    client.subscribe_many(&["temperature", "device_list"], &[1, 0]);

    while client.is_connected()
    {
        let msg_recv = client.start_consuming();
        match msg_recv.recv()
        {
            Ok(x) =>
            {
                let msg = x.unwrap();
                let mqtt_update: Result<MQTTUpdate, Box<bincode::ErrorKind>> = bincode::deserialize(&msg.payload());
                match mqtt_update
                {
                    Ok(dev_update) => update_device(dev_update),
                    Err(_) =>
                    {
                        //let device_list: Result<MQTTList, Box<bincode::ErrorKind>> = bincode::deserialize(&msg.payload()).unwrap_or_else(continue);
                    }
                }

                
    
                
    
            }
            Err(err) => 
            {
                println!("MQTT Server died, exiting");
                return;
            },
            
        } 
    }

}


fn device_list()
{

}

fn update_device(update: MQTTUpdate)
{

}