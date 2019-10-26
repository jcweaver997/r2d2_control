mod r2d2_connection;
use r2d2_connection::*;
use std::mem::transmute;
use std::io;
use std::io::prelude::*;
use gilrs::*;
use parking_lot::RwLock;
use std::sync::Arc;
use std::thread;

static R2D2_HOSTNAME:&str = "jc-desktop.local";
static R2D2_PORT:u16 = 1296;

fn recv(code:u8,values:&[u8;4]){
    println!("got message {},{:?}",code,values);
}

fn main() {
    let mut connection = R2D2Connection::new();
    connection.connect(R2D2_HOSTNAME,R2D2_PORT,ConnectionType::Client);
    let mut server = R2D2Connection::new();
    server.connect("0.0.0.0",1296,ConnectionType::Server);
    server.set_recv_fn(recv);
    connection.add(2,unsafe{&transmute(5_u32)});
    connection.add(3,unsafe{&transmute(5_u32)});
    connection.add(4,unsafe{&transmute(5_u32)});
    connection.send();



    let mut gilrs = Gilrs::new().unwrap();
    println!("Pick a controller:");
    let mut gamepadId = None;
    for (id, gamepad) in gilrs.gamepads(){
        println!("{}. {}", id,gamepad.name());
    }
    print!("Selection: ");
    io::stdout().flush().expect("failed to flush stdout");
    let input = io::stdin().lock().lines().next().expect("EOF").expect("?");
    let index: u32 = input.parse().expect("Not a number");
    for (id, gp) in gilrs.gamepads(){
        if id.to_string() == index.to_string(){
            gamepadId = Some(id);
        }
    }

    loop{
        while gilrs.next_event().is_some() {}

        if let Some(id) = gamepadId{
            println!("sd {:?}",gilrs.gamepad(id).is_pressed(Button::DPadRight));
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
    }


}
