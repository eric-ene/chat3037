use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use chat_shared::packet::ProcessedPacket;

pub type StreamType = Arc<Mutex<Option<TcpStream>>>;

pub struct Session {
  
  pub stream: StreamType,
  pub incoming: Arc<Mutex<VecDeque<ProcessedPacket>>>
}

impl Session {
  pub fn new(stream: StreamType, incoming: Arc<Mutex<VecDeque<ProcessedPacket>>>) -> Self {
    return Self {
      stream,
      incoming
    };
  }
}
