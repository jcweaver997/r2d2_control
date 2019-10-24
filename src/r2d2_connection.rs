use parking_lot::{Mutex,RwLock};
use std::sync::Arc;
use std::net::{UdpSocket, Ipv4Addr};

struct R2D2ConnectionData{
    port:Arc<Mutex<u16>>,
    hostname:Arc<Mutex<String>>,
    socket:Arc<RwLock<UdpSocket>>
}

pub struct R2D2Connection{
    connection_data:Option<R2D2ConnectionData>
}

impl R2D2Connection{
    pub fn new(hostname:&str,port:u16) -> R2D2Connection{
        println!("started discovery for {}",hostname);




        R2D2Connection{connection_data:None}
    }

    fn get_ip_from_mdns(service_name:&str) -> Option<Ipv4Addr>{
        let mut discovery = mdns::discover::all(hostname).expect("mDNS failure");
        discovery = discovery.timeout(std::time::Duration::from_secs(5));
        let result = discovery.next();
        if !result.is_some(){return None;}
        let ip = result.unwrap().records().find(|x| if)
        println!("mDNS result: {:?}",result);
        return None;
    }

    pub fn connect(){

    }
}