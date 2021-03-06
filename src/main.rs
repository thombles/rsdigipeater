extern crate ax25;
extern crate chrono;

use std::io;
use chrono::prelude::*;
use ax25::linux::{Ax25RawSocket, NetDev};
use ax25::frame::Ax25Frame;

fn main() {
    match run() {
        Ok(()) => (),
        Err(e) => println!("Error: {}", e)
    }
}

fn run() -> io::Result<()> {
    let socket = Ax25RawSocket::new()?;
    let devices = socket.list_ax25_interfaces()?;
    
    println!("Listening on devices:");
    for d in &devices {
        println!("* {}", d.name);
    }

    loop {
        match socket.receive_frame() {
            Ok(frame) => handle_frame(frame, &devices, &socket),
            Err(e) => return Err(e)
        }
    }
}

fn handle_frame(frame: Vec<u8>, devices: &Vec<NetDev>, socket: &Ax25RawSocket) {
    if let Ok(mut parsed) = Ax25Frame::from_bytes(&frame) {
        let mut transmit_ifindex: Option<i32> = None;
        if let Some(mut next_hop) = parsed.route.iter_mut().skip_while(|entry| entry.has_repeated ).next() {
            let addr_str = format!("{}", next_hop.repeater);
            for d in devices {
                if d.name == addr_str {
                    // Flip the bit to say it's been repeated before we transmit
                    next_hop.has_repeated = true;
                    transmit_ifindex = Some(d.ifindex);
                }
            }
        }
        if let Some(ifindex) = transmit_ifindex {
            let time = Local::now();
            let timestamp = time.format("%Y-%m-%d %H:%M:%S").to_string();
            println!("{} from {} to {} len {}", timestamp, parsed.source, parsed.destination, frame.len());
            if let Err(e) = socket.send_frame(&parsed.to_bytes(), ifindex) {
                println!("Error transmitting: {}", e);
            }
        }
    }
}
