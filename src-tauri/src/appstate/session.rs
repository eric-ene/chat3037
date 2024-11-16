use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::net::TcpStream;
use crate::helpers::packet::{Packet, ProcessedPacket};

pub struct Session {
  pub incoming: Arc<Mutex<VecDeque<ProcessedPacket>>>
}

impl Session {
  pub fn new(incoming: Arc<Mutex<VecDeque<ProcessedPacket>>>) -> Self {
    return Self {
      incoming
    };
  }
}
