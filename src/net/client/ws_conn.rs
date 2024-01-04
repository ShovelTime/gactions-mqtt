pub mod messaging{

    static HEARTBEAT_DELAY : Duration = Duration::from_secs(5);
    static TIMEOUT_DELAY : Duration = Duration::from_secs(10);
    // Open WS connection;

    use std::{time::{Duration, Instant}, ops::Deref, collections::HashMap, sync::{RwLock, Arc}};

    use actix_web::web::Data;
    use actix_web_actors::ws::{self, WebsocketContext, WsResponseBuilder};
    use actix::{Actor, StreamHandler, AsyncContext, ActorContext, Addr, SpawnHandle, Handler, WeakAddr};
    use actix_web::{web, Error, HttpRequest, HttpResponse, http::StatusCode};
    use chrono::{DateTime, Utc, FixedOffset, Local};
    use tokio::sync::broadcast::{Receiver, self};

    use crate::{net::client::ws_msg::ws_msg::{WsMessage, WsMessageType, PayloadDeviceUpdate, PayloadGetValue, PayloadScenarioUpdate, PayloadScenarioTimedToggle}, device::device::Device, home::scenarios::scenarios::TimedToggle, CONN_LIST};

    pub struct WsConn
    {
        hb : Instant,
        continuation_buf : Vec<u8>,
        self_addr : Option<WeakAddr<Self>>,
        dev_hash : Arc<RwLock<HashMap<String, Vec<Device>>>>,
        conn_list : Arc<RwLock<Vec<WeakAddr<Self>>>>


    }
    
    impl WsConn{

        pub fn new(dev_hash: Arc<RwLock<HashMap<String, Vec<Device>>>>, conn_list: Arc<RwLock<Vec<WeakAddr<Self>>>>) -> WsConn
        {
            WsConn{hb: Instant::now(), conn_list, self_addr : None, continuation_buf: Vec::new(), dev_hash}
            
        }
    }

    impl Actor for WsConn {
        type Context = ws::WebsocketContext<Self>;
    
        fn started(&mut self, ctx: &mut Self::Context) {
            self.self_addr = Some(ctx.address().downgrade());
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
    
    impl Handler<WsMessage> for WsConn{
        type Result = Result<(), serde_json::Error>;

        fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) -> Self::Result {
            let res = serde_json::to_string(&msg);
            match res{
                Ok(msg_str) => {
                    ctx.text(msg_str);
                    return Ok(())
                },
                Err(err) => return Err(err),
            }
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
                                                            tgt_dev.update(&payload.device);
                                                            //TODO:Inform every connected client of
                                                            //change
                                                            

                                                },
                                                Err(_) => println!("Error deserializing device update!"),
                                            }
                                        },
                                        WsMessageType::SCENARIO_UPDATE => {
                                            match serde_json::from_str::<PayloadScenarioUpdate>(&wsmsg.payload)
                                            {
                                                Ok(scenario) => {
                                                    match scenario.scenario_type {
                                                        crate::automatisation::voice_recognition::voice_recognition::ScenarioTypes::TIMED => {
                                                            let Ok(s_payload) = serde_json::from_str::<PayloadScenarioTimedToggle>(&scenario.scenario_payload) else {ctx.text("Malformed Scenario Payload!"); return};
                                                            match self.dev_hash.read(){
                                                                Ok(lock) => {
                                                                   let Some(dev) = lock.values().flatten().find(|d| *d == s_payload.sensor_id) else {ctx.text("device_id not found!"); return}; 
                                                                   let id = dev.get_id().clone(); 
                                                                   drop(lock); 
                                                                   let Ok(time) = DateTime::<FixedOffset>::parse_from_rfc3339(&s_payload.time) else {ctx.text(format!("Malformed time string! {}", s_payload.time)); return};//TimedToggle
                                                                   
                                                                   let Ok(timestamp) = TryInto::<u64>::try_into(time.naive_local().timestamp() - Local::now().naive_local().timestamp()) else {ctx.text("target time cant be before the present!"); return};
                                                                   let Some(time_until) = std::time::Instant::now().checked_add(Duration::from_secs(timestamp)) else {ctx.text("Instant got out of range!"); return}; 
                                                                   TimedToggle::new(actix::clock::Instant::from_std(time_until), vec!(id), self.dev_hash.clone(), self.conn_list.clone());
                                                                        
                                                                },
                                                                Err(_) => panic!("Whoever panicked above us is a poisonous nerd"),
                                                            }
                                                        },
                                                        crate::automatisation::voice_recognition::voice_recognition::ScenarioTypes::SENSOR_CONDITIONAL => todo!(),
                                                        crate::automatisation::voice_recognition::voice_recognition::ScenarioTypes::READ_SENSOR_OR_STATE => todo!(),
                                                        crate::automatisation::voice_recognition::voice_recognition::ScenarioTypes::GENERAL_KENOBI => todo!(),
                                                    }
                                                },
                                                Err(_) => todo!(),
                                            }
                                        },
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
                Err(err) =>
                {
                        match err
                        {
                                ws::ProtocolError::UnmaskedFrame => todo!(),
                                ws::ProtocolError::MaskedFrame => todo!(),
                                ws::ProtocolError::InvalidOpcode(_) => todo!(),
                                ws::ProtocolError::InvalidLength(_) => todo!(),
                                ws::ProtocolError::BadOpCode => todo!(),
                                ws::ProtocolError::Overflow => todo!(),
                                ws::ProtocolError::ContinuationNotStarted => todo!(),
                                ws::ProtocolError::ContinuationStarted => todo!(),
                                ws::ProtocolError::ContinuationFragment(_) => todo!(),
                                ws::ProtocolError::Io(_) => todo!(),
                        }
             
                }
        }

    }

}

    pub fn send_ws_message(conn_list : Arc<RwLock<Vec<WeakAddr<WsConn>>>>, msg : WsMessage)
    {
        
        match crate::CONN_LIST.write()
        {
            Ok(lock) => {
                for w_addr in lock.iter()
                    {
                        let Some(addr) = w_addr.upgrade() else {
                            println!("Addr {:?} was not cleared!", w_addr);
                            continue};
                        addr.do_send(msg.clone());
                    }
        },
        Err(_) => todo!(),
        }
    }

    pub async fn send_ws_message_async(conn_list : Arc<RwLock<Vec<WeakAddr<WsConn>>>>, msg : WsMessage)
    {
        send_ws_message(conn_list, msg)
    }

    
    async fn ws_conn_request(
        req: HttpRequest,
        stream: web::Payload,
        device_lock : Data<Arc<RwLock<HashMap<String, Vec<Device>>>>>, 
        ws_handler: Data<Arc<RwLock<Vec<WeakAddr<WsConn>>>>>
    ) -> Result<HttpResponse, Error>
    {
        let ws_instance = WsConn::new(Arc::clone(device_lock.clone().deref()),
            Arc::clone(ws_handler.clone().deref()));
        let ws_conn = WsResponseBuilder::new(ws_instance, &req, stream);
        
        match ws_conn.start_with_addr()
        {
            Ok(res) => {
                    CONN_LIST.write().expect("Alright, who's the funny thread that poisoned our lock, WHILE LEAVING?!").push(res.0.downgrade());
                    return Ok(res.1);
                },
            Err(_) => return Ok(HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }

}
