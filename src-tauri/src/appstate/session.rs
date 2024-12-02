use std::collections::VecDeque;
use std::sync::{Arc, LockResult, Mutex};
use std::net::TcpStream;
use std::time::{Duration, Instant};
use chat_shared::packet::{Packet, PacketType, ProcessedPacket};
use crate::helpers::shared_tools::SharedVec;

pub type StreamType = Arc<Mutex<Option<Arc<Mutex<TcpStream>>>>>;

pub struct Session {
  
  pub stream: StreamType,
  pub incoming: SharedVec<ProcessedPacket>
}

impl Session {
  pub fn new(stream: StreamType, incoming: SharedVec<ProcessedPacket>) -> Self {
    return Self {
      stream,
      incoming
    };
  }
  
  pub fn remove_first(&self, packet_type: PacketType) -> Option<ProcessedPacket> {
    let mut incoming = match self.incoming.lock() {
      Ok(guard) => guard,
      Err(_) => return None,
    };
    
    let mut rm = None;
    
    for (index, packet) in incoming.iter().enumerate() {
      if packet == packet_type {
        rm = Some(index);
      }
    }
    
    let packet = match rm {
      None => return None,
      Some(idx) => match incoming.remove(idx) {
        None => return None,
        Some(packet) => packet,
      }
    };

    return Some(packet);
  }
  
  pub fn wait_and_remove_first(&self, packet_type: PacketType, timeout: Duration) -> Result<ProcessedPacket, String> {
    let start = Instant::now();
    
    while start.elapsed() < timeout {
      match self.remove_first(packet_type) {
        Some(packet) => return Ok(packet),
        None => continue
      }
    }
    return Err(format!("Timed out after: {}", start.elapsed().as_secs()));
  }
}

