pub mod simmed
{
    use core::time;
    use std::thread;

    use crate::{devices::traits::devices::Device, net::device_update::device_updates::MQTTDevice};
    use mqtt::{ConnectOptions, MessageBuilder};
    use paho_mqtt as mqtt;
    use serde::Serialize;

    pub fn simulate_devices(device_list: Vec<MQTTDevice>)
    {
        
        let broker_conn = mqtt::Client::new("tcp://localhost:1883").unwrap();
        let conn_options : ConnectOptions = ConnectOptions::new_v5();
        let token = broker_conn.connect(conn_options).unwrap();

        let list_message = MessageBuilder::default()
        .qos(1)
        .retained(true)
        .topic("device_list")
        .payload(bincode::serialize(&device_list).unwrap())
        .finalize();

        broker_conn.publish(list_message).unwrap();
        
        loop{
            thread::sleep(time::Duration::from_secs(3));
            
        }


    }


}