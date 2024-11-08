use std::net::UdpSocket;
use std::time::Duration;
use anyhow::anyhow;
use clap::Parser;

use rand::random;
use stun_client::{Attribute, Class, Client};

#[derive(Parser)]
struct Cli {
    /// IPv4 address of the other client
    #[arg(short, long)]
    dst: String,
    /// Port to use
    #[arg(short, long, default_value_t = 25957)]
    port: u16,
}

const STUN_SERVER: &str = "stun3.l.google.com:19302";
const MAGIC_COOKIE: u32 = 0x2112A442;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // let args = Cli::parse();
    
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_read_timeout(Some(Duration::from_secs_f32(3.0)))?;
    
    let transaction_id = random::<[u8; 12]>();
    let request = stun_request(1, 0, transaction_id);
    
    
    socket.send_to(&request, STUN_SERVER)?;
    
    let (ip, port) = parse_stun(socket, MAGIC_COOKIE.to_be_bytes());
    println!("stun response: {}:{}", ip, port);
    
    Ok(())
}

fn parse_stun(socket: UdpSocket, tid: [u8; 4]) -> (String, u16) {
    let mut response = [0_u8; 512];
    let _ = socket.recv(&mut response).expect("error receiving response from STUN server");
    
    let res = &response[26..32];
    
    let ip = format!("{}.{}.{}.{}", res[2] ^ tid[0], res[3] ^ tid[1], res[4] ^ tid[2], res[5] ^ tid[3]);
    let port = u16::from_be_bytes([res[0] ^ tid[0], res[1] ^ tid[1]]);
    
    return (ip, port);
}

fn stun_request(ty: u16, length: u16, id: [u8; 12]) -> Vec<u8> {
    let mut retval = Vec::new();
    
    for byte in ty.to_be_bytes() {
        retval.push(byte);
    }

    for byte in length.to_be_bytes() {
        retval.push(byte);
    }

    // magic cookie
    for byte in 0x2112A442_u32.to_be_bytes() {
        retval.push(byte);
    }
    
    for byte in id {
        retval.push(byte);
    }
    
    return retval;
}
