use crate::appstate::session;
use crate::network::stream::StreamThreadTools;
use std::collections::VecDeque;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpStream};
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::Duration;
use chat_shared::packet::{Packet, PacketSymbols, ProcessedPacket};
use chat_shared::packet::assign::AssignRequestPacket;
use chat_shared::packet::handshake::HandshakeStatus;
use chat_shared::stream::read::{ReadError, ReadUntil};
use tauri::{App, AppHandle, Emitter};
use crate::helpers::events::HandshakeEvent;

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
    println!("connected to server.");
    
    // get lock on mutex
    let mut guard = stream_arc.lock().unwrap_or_else(|e| { 
      println!("Error locking mutex: {:?}", e);
      e.into_inner() 
    });
    
    println!("stream assigned.");
    
    *guard = Some(stream);
  });
}

pub fn start_listener(
  app: AppHandle,
  mut stream: session::StreamType,
  incoming: Arc<Mutex<VecDeque<ProcessedPacket>>>
) -> JoinHandle<()> {
  return thread::spawn(move || {
    stream.block_exec(|stream| {
      let request_packet = AssignRequestPacket {};
      let mut request_raw = ProcessedPacket::new_raw(ProcessedPacket::AssignRequest(request_packet));
      let mut packet = request_raw.as_slice();
      
      loop {
        match stream.write_all(packet) {
          Ok(_) => break,
          Err(e) => {
            println!("couldn't send packet! {}", e);
            continue;
          }
        }
      }
    });
    
    stream.block_exec(|stream| stream.set_nonblocking(true).unwrap());

    let buf = Arc::new(Mutex::new(Vec::new()));
    let timeout = Duration::from_secs_f32(0.5);
    
    loop {
      stream.block_exec(|stream| {
        let mut buf = buf.lock().unwrap();
        
        let n_read = match stream.read_until_timeout(&mut buf, PacketSymbols::Eom, timeout) {
          Ok(n) => n,
          Err(e) => match e {
            ReadError::Timeout => return,
            ReadError::Other(e) => {
              println!("couldn't read from stream! {}", e);
              *buf = Vec::new();
              return;
            }
          }
        };

        if n_read == 0 {
          return;
        }

        if n_read < 8 {
          println!("mangled packet! size {} is too small!", n_read);
          return;
        }

        let packet = Packet::from_bytes(&mut buf);
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

        println!("received packet {:?}", processed);
        
        match processed {
          ProcessedPacket::Handshake(pack) => match pack.status {
            HandshakeStatus::Request => { // needs to be handled immediately
              let payload = HandshakeEvent {
                status: HandshakeStatus::Request,
                sender: format!("{}", pack.src),
              };
              
              println!("emitting event");
              let _ = app.emit("handshake", payload);
            }
            _ => incoming.push_back(ProcessedPacket::Handshake(pack))
          }
          pack => incoming.push_back(pack)
        };
      });
    }
  });
}