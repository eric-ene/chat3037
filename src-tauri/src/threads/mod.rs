use std::collections::VecDeque;
use std::io::Read;
use std::net::TcpStream;
use std::sync::{Arc, LockResult, Mutex};
use std::thread;
use std::thread::JoinHandle;
use crate::helpers::packet::{Packet, ProcessedPacket};

pub fn connect_and_listen(ip: String, incoming: Arc<Mutex<VecDeque<ProcessedPacket>>>) -> JoinHandle<()> {
  return thread::spawn(move || {
    let mut stream = TcpStream::connect(ip.as_str());
    while stream.is_err() {
      stream = TcpStream::connect(ip.as_str())
    }
    
    match stream {
      Ok(stream) => {
        start_listener(stream, incoming);
      }
      Err(e) => {
        println!("unreachable error: {}", e);
      }
    }
  });
}

pub fn start_listener(mut stream: TcpStream, incoming: Arc<Mutex<VecDeque<ProcessedPacket>>>) {
  loop {
    let mut buf = Vec::new();
    let n_read = match stream.read_to_end(&mut buf) {
      Ok(val) => val,
      Err(e) => {
        println!("error reading from stream: {}", e);
        continue;
      }
    };
    
    if n_read == 0 {
      continue;
    }
    
    if n_read < 8 {
      println!("mangled packet! size {} is too small!", n_read);
      continue;
    }
    
    let packet = Packet::from_bytes(buf);
    let mut incoming = match incoming.lock() {
      Ok(mut guard) => guard,
      Err(e) => continue
    };
    
    println!("{:?}", packet);
    let processed = match packet.process() {
      Ok(val) => val,
      Err(e) => {
        println!("couldn't process packet: {}", e);
        continue;
      }
    };
    
    incoming.push_back(processed);
  }
}