use crate::appstate::session;
use crate::network::stream::StreamThreadTools;
use std::collections::VecDeque;
use std::io::Read;
use std::net::TcpStream;
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::Duration;
use chat_shared::packet::{Packet, ProcessedPacket};

pub fn connect_init(ip: &'static str, stream_arc: session::StreamType) -> JoinHandle<()> {
  return thread::spawn(move || {
    // connect and return stream
    let stream = loop {
      let stream = TcpStream::connect(ip);
      match stream {
        Ok(stream) => break stream,
        Err(e) => {
          println!("Error connecting to server: {:?}", e);
          sleep(Duration::from_millis(100));
          continue;
        }
      }
    };
    
    // get lock on mutex
    let mut guard = stream_arc.lock().unwrap_or_else(|e| { 
      println!("Error locking mutex: {:?}", e);
      e.into_inner() 
    });
    
    *guard = Some(stream);
  });
}

pub fn start_listener(mut stream: session::StreamType, incoming: Arc<Mutex<VecDeque<ProcessedPacket>>>) -> JoinHandle<()> {
  return thread::spawn(move || {
    loop {
      stream.block_exec(|stream| {
        let mut buf = Vec::new();
        
        let n_read = match stream.read_to_end(&mut buf) {
          Ok(val) => val,
          Err(e) => {
            println!("error reading from stream: {}", e);
            return;
          }
        };

        if n_read == 0 {
          return;
        }

        if n_read < 8 {
          println!("mangled packet! size {} is too small!", n_read);
          return;
        }

        let packet = Packet::from_bytes(buf);
        let mut incoming = match incoming.lock() {
          Ok(mut guard) => guard,
          Err(e) => return
        };
        
        let processed = match packet.process() {
          Ok(val) => val,
          Err(e) => {
            println!("couldn't process packet: {}", e);
            return;
          }
        };

        incoming.push_back(processed);

        sleep(Duration::from_millis(20));
      });
    }
  });
}