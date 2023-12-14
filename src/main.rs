use actix_web::{HttpServer, App};
use tokio::net::TcpListener;
use paho_mqtt::AsyncClient;

pub mod automatisation;
pub mod home;
pub mod net;
pub mod devices;

#[tokio::main(worker_threads = 8)]
async fn main() {
    println!("Hello, world!");

    let listener = TcpListener::bind("0.0.0.0:80").await.expect("Failed to bind Socket"); // replace port with something else;

    HttpServer::new(move || {App::new()});

}
