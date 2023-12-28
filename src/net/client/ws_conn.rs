pub mod messaging{

    static HEARTBEAT_DELAY : Duration = Duration::from_secs(5);
    static TIMEOUT_DELAY : Duration = Duration::from_secs(10);
    // Open WS connection;

    use std::{time::{Duration, Instant}, ops::Deref, collections::HashMap, sync::{RwLock, Arc}};

    use actix_web::web::Data;
    use actix_web_actors::ws;
    use actix::{Actor, StreamHandler, AsyncContext, ActorContext, Addr};
    use actix_web::{web, Error, HttpRequest, HttpResponse, http::StatusCode};

    use crate::{net::client::ws_msg::ws_msg::{WsMessage, WsMessageType, PayloadDeviceUpdate, PayloadGetValue}, device::device::Device};

    struct WsConn
    {
        hb : Instant,
        continuation_buf : Vec<u8>,
        dev_hash : Arc<RwLock<HashMap<String, Vec<Device>>>>
    }
    
    impl WsConn{

        pub fn new(dev_hash: Arc<RwLock<HashMap<String, Vec<Device>>>>) -> WsConn
        {
            WsConn{hb: Instant::now(), continuation_buf: Vec::new(), dev_hash}
            
        }
    }

    impl Actor for WsConn {
        type Context = ws::WebsocketContext<Self>;
    
        fn started(&mut self, ctx: &mut Self::Context) {
            ctx.run_interval(HEARTBEAT_DELAY, |act, ctx| {
                if Instant::now().duration_since(act.hb) > TIMEOUT_DELAY
                {
                    ctx.text("Heartbeat interval has expired! terminating connection.");
                    ctx.stop();
                    return;
                }
                ctx.ping(b"ping");
                
            });
        }
        
        fn stopped(&mut self , _: &mut Self::Context)
        {
            println!("WS connection has been terminated!");
        }
        
    
    }

    impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
        fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
            match msg {
                Ok(res) => {
                    match res {
                        ws::Message::Ping(msg) => {
                            self.hb = Instant::now();
                            ctx.pong(&msg)
                        },
                        ws::Message::Pong(_) =>
                        {
                            self.hb = Instant::now();
                        }
                        ws::Message::Binary(bytes)=> {
                            
                            let dat_slice: &[u8] = bytes.deref();
                            match serde_json::from_slice::<WsMessage>(dat_slice)
                            {
                                Ok(wsmsg) => {
                                    match wsmsg.message_type
                                    {
                                        WsMessageType::DEVICE_CMD => {
                                                
                                                } ,
                                        WsMessageType::DEVICE_UPDATE => {
                                            match serde_json::from_str::<PayloadDeviceUpdate>(&wsmsg.payload){
                                                Ok(payload) => {
                                                            let Ok(mut map) = self.dev_hash.write() else {return};
                                                            let Some(dev) = map.get_mut(&payload.device.topic) else {return};
                                                            let Some(tgt_dev) = dev.iter_mut().find(|d| **d == payload.device) else {return};
                                                            tgt_dev.update(payload.device);

                                                },
                                                Err(_) => println!("Error deserializing device update!"),
                                            }
                                        },
                                        WsMessageType::SCENARIO_UPDATE => todo!(),
                                        WsMessageType::VALUE_GET => {
                                            match serde_json::from_str::<PayloadGetValue>(&wsmsg.payload){
                                                Ok(payload) => {
                                                            let Ok(map) = self.dev_hash.read() else {return};
                                                            let Some(dev) = map.get(&payload.topic) else {return};
                                                            let Some(tgt_dev) = dev.iter().find(|d| **d == payload.device_id) else {return};
                                                            let val = tgt_dev.get_value().unwrap_or("null".to_string());
                                                            ctx.text(val);
                                                            
                                                },
                                                Err(_) => println!("Error deserializing device update!"),
                                            } 
                                        },
                                        _ => ctx.text("Invalid Message Type!")
                                    }
                                },
                                Err(e) => 
                                {
                                    ctx.text("Message Serialization Error.")

                                }
                            }

                        },
                        ws::Message::Continuation(cont) => todo!(), 
                        ws::Message::Close(opt) =>
                        {
                            ctx.close(opt)
                        }
                        ws::Message::Text(text) => todo!(),
                        ws::Message::Nop => (), // Wat
                    }
                }
                Err(_) => todo!(),
            }
        }
    }


    async fn ws_conn_request(
        req: HttpRequest,
        stream: web::Payload,
        device_lock : Data<Arc<RwLock<HashMap<String, Vec<Device>>>>>,
    
    ) -> Result<HttpResponse, Error>
    {
        let ws_instance = WsConn::new(device_lock.into_inner().deref().clone());
        match ws::start(ws_instance, &req, stream)
        {
            Ok(res) => return Ok(res),
            Err(_) => return Ok(HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}
