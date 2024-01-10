pub mod messaging{

    static HEARTBEAT_DELAY : Duration = Duration::from_secs(5);
    static TIMEOUT_DELAY : Duration = Duration::from_secs(10);
    // Open WS connection;

    use std::{time::{Duration, Instant}, ops::{Deref, Range}};

    
    use actix_web_actors::ws::{self, WsResponseBuilder};
    use actix::{Actor, StreamHandler, AsyncContext, ActorContext, Handler, WeakAddr};
    use actix_web::{web::{self, Bytes, Data}, Error, HttpRequest, HttpResponse, http::StatusCode};
    use chrono::{DateTime, FixedOffset, Local};
    use serde_json::{Map, Value};
    use tokio::sync::mpsc::UnboundedSender;
    

    use crate::{net::{client::ws_msg::ws_msg::{WsMessage, WsMessageType, PayloadDeviceUpdate, PayloadGetValue, PayloadScenarioUpdate, PayloadScenarioTimedToggle, PayloadDeviceCommand, CommandType, PayloadScenarioSensorConditional}, device_update::device_updates::{MQTTUpdate, DeviceUpdateType}}, home::scenarios::scenarios::{TimedToggle, ConditionalTrigger}, CONN_LIST, ws_error, DEVICE_CONTAINER, automatisation::voice_recognition::voice_recognition::ScenarioTypes, SCENARIO_LIST, device::device::Device};
    pub struct WsConn
    {
        hb : Instant,
        //continuation_buf : Vec<u8>,
        self_addr : Option<WeakAddr<Self>>,
        tx : UnboundedSender<MQTTUpdate>
    }
    
    impl WsConn{

        pub fn new(tx: UnboundedSender<MQTTUpdate>) -> WsConn
        {
            WsConn{hb: Instant::now(),  self_addr : None, tx 
                //continuation_buf: Vec::new()
            }
            
        }
        fn handle_payload(&mut self, bytes : Bytes, ctx : &mut <Self as Actor>::Context)
        {
                            let dat_slice: &[u8] = bytes.deref();
                            println!("message received! \n {:?}", serde_json::from_slice::<WsMessage>(dat_slice).unwrap());
                            match serde_json::from_slice::<WsMessage>(dat_slice)
                            {
                                Ok(wsmsg) => {
                                    match wsmsg.message_type
                                    {
                                        WsMessageType::DEVICE_CMD => {
                                            match serde_json::from_str::<PayloadDeviceCommand>(&wsmsg.payload){
                                                Ok(payload) => {
                                                    let Ok(mut map) = DEVICE_CONTAINER.write() else {println!("hashmap poisoned!"); return};
                                                    let Some(tgt_dev) = map.values_mut().flatten().find(|d| {*d.get_id() == payload.device_id}) else {println!("Failed to find device : {}!, \n {:?}, \n {:?}", payload.device_id, payload, map.values().flatten().collect::<Vec<&Device>>()); return};
                                                    match payload.command{
                                                        CommandType::TOGGLE => tgt_dev.toggle(),
                                                        CommandType::ENABLE => tgt_dev.activated = true,
                                                        CommandType::DISABLE => tgt_dev.activated = false,
                                                        CommandType::UNKNOWN => {ctx.text(ws_error!(format!("Unknown Command Type! {}", wsmsg.payload))); return},
                                                    }
                                                    let n_msg = WsMessage::device_update(tgt_dev).expect("device parsing failed");
                                                    println!("Sending Device Update Payload after command, \n {:?}", n_msg);

                                                    
                                                    let mut mqtt_map = Map::<String, Value>::new();
                                                    mqtt_map.insert("activated".to_string(), tgt_dev.activated.to_string().into());
                                                    let update = MQTTUpdate{ update_type: DeviceUpdateType::ACTIVATION_CHANGE, device_id: payload.device_id, topic:tgt_dev.topic.clone() , update_fields: mqtt_map };
                                                    drop(map);
                                                    let _ = self.tx.send(update);
                                                    send_ws_message(n_msg); 


                                                },
                                                Err(_) => todo!(),
                                            }
                                                
                                        } ,
                                        WsMessageType::DEVICE_UPDATE => {
                                            println!("deprecated function called");
                                            match serde_json::from_str::<PayloadDeviceUpdate>(&wsmsg.payload){
                                                Ok(payload) => {
                                                            let Ok(mut map) = DEVICE_CONTAINER.write() else {return};
                                                            let dev = {
                                                                if !map.contains_key(&payload.device.topic)
                                                                {
                                                                    map.insert(payload.device.topic.clone(), Vec::new());
                                                                }
                                                                map.get_mut(&payload.device.topic).unwrap()

                                                            };
                                                            let Some(tgt_dev) = dev.iter_mut().find(|d| **d == payload.device) else {return};
                                                            tgt_dev.update(&payload.device);
                                                            drop(map);
                                                            send_ws_message(wsmsg);
                                                },
                                                Err(_) => println!("Error deserializing device update!"),
                                            }
                                        },
                                        WsMessageType::SCENARIO_UPDATE => {
                                            match serde_json::from_str::<PayloadScenarioUpdate>(&wsmsg.payload)
                                            {
                                                Ok(scenario) => {
                                                    match scenario.scenario_type {
                                                        ScenarioTypes::TIMED => {
                                                            let Ok(s_payload) = serde_json::from_str::<PayloadScenarioTimedToggle>(&scenario.scenario_payload) else 
                                                                        {ctx.text(ws_error!("Malformed Scenario Payload!")); return};
                                                            match DEVICE_CONTAINER.read(){
                                                                Ok(lock) => {
                                                                   let Some(dev) = lock.values().flatten().find(|d| *d == s_payload.sensor_id) else 
                                                                                {ctx.text(ws_error!("device_id not found!")); return}; 
                                                                   let id = dev.get_id().clone(); 
                                                                   drop(lock); 
                                                                   let Ok(time) = DateTime::<FixedOffset>::parse_from_rfc3339(&s_payload.time) else 
                                                                                {ctx.text(ws_error!(format!("Malformed time string! {}", s_payload.time))); return};//TimedToggle
                                                                   
                                                                   let Ok(timestamp) = TryInto::<u64>::try_into(time.naive_local().timestamp() - Local::now().naive_local().timestamp()) else 
                                                                                {ctx.text(ws_error!("target time cant be before the present!")); return};
                                                                   let Some(time_until) = std::time::Instant::now().checked_add(Duration::from_secs(timestamp)) else 
                                                                                {ctx.text(ws_error!("Instant got out of range!")); return}; 
                                                                   let s_id = TimedToggle::new(actix::clock::Instant::from_std(time_until), vec!(id), self.tx.clone());

                                                                   let mut s_res = scenario.clone(); 
                                                                   s_res.scenario_id = Some(s_id);
                                                                   s_res.completed = Some(false);
                                                                    
                                                                   send_ws_message( WsMessage{
                                                                       message_type: WsMessageType::SCENARIO_UPDATE,
                                                                       payload : serde_json::to_string(&s_res).expect("failed to serialize scenario update")

                                                                   });

                                                                   SCENARIO_LIST.read().expect("failed to get scenario list").iter().find(|s| {s.get_id() == s_id}).unwrap().start();
                                                                        
                                                                },
                                                                Err(_) => panic!("Whoever panicked above us is a poisonous nerd"),
                                                            }
                                                        },
                                                        ScenarioTypes::SENSOR_CONDITIONAL =>
                                                        {
                                                            let Ok(s_payload) = serde_json::from_str::<PayloadScenarioSensorConditional>(&scenario.scenario_payload) else 
                                                                        {ctx.text(ws_error!(format!("Malformed Scenario Payload! \n {}", scenario.scenario_payload))); return};
                                                            
                                                            let range: Range<i32>;
                                                            if s_payload.cmp_over
                                                            {
                                                                range = s_payload.treshold..i32::MAX;
                                                            }
                                                            else
                                                            {
                                                                range = i32::MIN..s_payload.treshold;
                                                            }

                                                            let res = ConditionalTrigger::new(s_payload.sensor_id, range, s_payload.target_device, self.tx.clone());
                                                            match res {
                                                                Ok(s_id) => {
                                                                    let mut s_res = scenario.clone();
                                                                    s_res.scenario_id = Some(s_id);
                                                                    s_res.completed = Some(false);
                                                                    
                                                                    send_ws_message( WsMessage{
                                                                        message_type: WsMessageType::SCENARIO_UPDATE,
                                                                        payload : serde_json::to_string(&s_res).expect("failed to serialize scenario update")

                                                                    });


                                                                },
                                                                Err(err) => ctx.text(ws_error!(err)),
                                                            }


                                                        }

                                                        ScenarioTypes::READ_SENSOR_OR_STATE => todo!(),
                                                        ScenarioTypes::GENERAL_KENOBI => println!("hello there"), 
                                                    }
                                                },
                                                Err(_) => todo!(),
                                            }
                                        },
                                        WsMessageType::VALUE_GET => {
                                            match serde_json::from_str::<PayloadGetValue>(&wsmsg.payload){
                                                Ok(payload) => {
                                                            let Ok(map) = DEVICE_CONTAINER.read() else {return};
                                                            let Some(dev) = map.get(&payload.topic) else {ctx.text(ws_error!("Topic for device not found!")); return};
                                                            let Some(tgt_dev) = dev.iter().find(|d| **d == payload.device_id) else {ctx.text(ws_error!("Device not found!")); return};
                                                            let val = tgt_dev.get_value().unwrap_or("null".to_string());
                                                            send_ws_message(WsMessage::value(tgt_dev.get_id().to_string(), val).expect("how did 2 string parsing fail"))                                                            
                                                },
                                                Err(_) => println!("Error deserializing device update!"),
                                            } 
                                        },
                                        _ => ctx.text("Invalid Message Type!")
                                    }
                                },
                                Err(e) => 
                                {

                                    println!("Message Deserialization Error : {}", e.to_string());
                                    ctx.text(ws_error!(format!("Message Deserialization error: {}", e.to_string())))

                                }
                            }


        }
    }

    impl Actor for WsConn {
        type Context = ws::WebsocketContext<Self>;
    
        fn started(&mut self, ctx: &mut Self::Context) {
            println!("WS Connection Opened!");
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

            ctx.text(serde_json::to_string(&WsMessage::device_list(DEVICE_CONTAINER.read().unwrap().values().flatten().collect()).unwrap()).unwrap());
            //TODO: Fix sending Scenario List
            
                
            
        }
        
        fn stopped(&mut self , ctx: &mut Self::Context)
        {
            println!("WS connection has been terminated!");
            let mut lock = CONN_LIST.write().expect("conn lock is poisoned!");
            let Some(addr_pos) = lock.iter().position(|x| *x == self.self_addr.clone().unwrap_or(ctx.address().downgrade())) else 
            {return}; // I dont know how that got created without getting into the list but alright.
            lock.swap_remove(addr_pos); // we really dont care about the order so we can take the
            // small performance improvement
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
                            self.handle_payload(bytes, ctx);
                        },
                        ws::Message::Continuation(_cont) => todo!(), 
                        ws::Message::Close(opt) =>
                        {
                            ctx.close(opt)
                        }
                        ws::Message::Text(text) => {
                            self.handle_payload(text.as_bytes().clone(), ctx);

                        },
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

    pub fn send_ws_message( msg : WsMessage)
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

    pub async fn send_ws_message_async( msg : WsMessage)
    {
        send_ws_message( msg)
    }

    
    pub async fn ws_conn_request(
        req: HttpRequest,
        stream: web::Payload,
        tx_dat: Data<UnboundedSender<MQTTUpdate>>
    ) -> Result<HttpResponse, Error>
    {
        let ws_instance = WsConn::new(tx_dat.into_inner().deref().clone());

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
