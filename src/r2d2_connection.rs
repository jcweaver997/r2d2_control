use parking_lot::{RwLock};
use std::sync::Arc;
use std::net::{UdpSocket};
use std::thread;

#[derive(PartialEq,Copy,Clone)]
pub enum ConnectionType{
    Client,Server,None
}

pub struct R2D2Connection{
    socket:Option<Arc<RwLock<UdpSocket>>>,
    next_packet:(Vec<u8>,Vec<u8>),
    recv_fn:Arc<RwLock<Option<fn(u8,&[u8;4])>>>,
    connection_type:ConnectionType
}

impl R2D2Connection{
    pub fn new() -> R2D2Connection{
        R2D2Connection{
            socket:None,
            next_packet:(Vec::new(),Vec::new()),
            recv_fn:Arc::new(RwLock::new(None)),
            connection_type:ConnectionType::None
        }
    }

    pub fn connect(&mut self, hostname: &str, port: u16, con_type: ConnectionType){
            self.connection_type = con_type;
        if con_type == ConnectionType::Client{
            self.socket = Some(Arc::new(RwLock::new(UdpSocket::bind("0.0.0.0:0").expect("Failed to bind UDP socket"))));
            self.socket.as_ref().unwrap().write().connect(format!("{}:{}",hostname,port)).expect("Failed to connect UDP socket");
            self.start_listener();
        }else if con_type == ConnectionType::Server{
            self.socket = Some(Arc::new(RwLock::new(UdpSocket::bind(format!("0.0.0.0:{}",port)).expect("Failed to bind UDP socket"))));
            self.start_listener();
        }
    }

    pub fn set_recv_fn(&self, recv_fn:fn(u8,&[u8;4])){
        *self.recv_fn.write() = Some(recv_fn);
    }

    fn start_listener(&self){
        let socket = self.socket.as_ref().unwrap().clone();
        let recv_fn = self.recv_fn.clone();
        let connection_type = self.connection_type;
        thread::spawn(move ||{
            loop{
                let mut buffer = [0_u8;100];
                let (count, remote) = socket.read().recv_from(&mut buffer).expect("Failed to read from socket");
                if connection_type == ConnectionType::Server{socket.read().connect(remote).expect("Failed to connect socket")};
                if let Some(recv) = *recv_fn.read(){
                    if count%5 == 0{
                        let mut value = [0_u8;4];
                        for i in 0..count/5{
                            for j in 0..4{
                                value[j] = buffer[i*5+j+1];
                            }
                            recv(buffer[i*5],&value);
                        }
                    }
                }
            }
        });
    }

    pub fn add(&mut self, code:u8, value:&[u8;4]){
        self.next_packet.0.push(code.clone());
        self.next_packet.1.extend(value);
    }

    pub fn send(&mut self){
        if self.next_packet.0.len() == 0{return;}
        if let Some(socket) = &self.socket{
            let mut buffer = Vec::new();
            for i in 0..self.next_packet.0.len(){
                buffer.push(self.next_packet.0[i]);
                for j in 0..4{
                    buffer.push(self.next_packet.1[i*4+j]);
                }
            }
            socket.read().send(&buffer).expect("Failed to send packet");
            self.next_packet.0.clear();
            self.next_packet.1.clear();
        }
    }
}
