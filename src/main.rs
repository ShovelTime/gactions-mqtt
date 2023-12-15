use actix_web::{HttpServer, App};
use devices::traits::devices::Device;
use net::{simmed::simmed::simulate_devices, device_update::device_updates::{MQTTDevice}};
use tokio::net::TcpListener;
use paho_mqtt::AsyncClient;
use crate::devices::sensors::sensors::DeviceType;
use std::thread;

pub mod automatisation;
pub mod home;
pub mod net;
pub mod devices;

#[tokio::main(worker_threads = 8)]
async fn main() {

    let thread =thread::spawn(move || // Simulated Devices
    {

        let device = MQTTDevice{
            device : Device{
                device_type:DeviceType::TempSensor,
                name: "Temperature Sensor".to_string(),
                connected: true,
                activated: true,
                value: "15C".to_string() }
        };
        simulate_devices(vec!(device));
    });

    thread.join().unwrap();

    
/* 
    let listener = TcpListener::bind("0.0.0.0:80").await.expect("Failed to bind Socket"); // replace port with something else;

    HttpServer::new(move || {App::new()});
    */

}
