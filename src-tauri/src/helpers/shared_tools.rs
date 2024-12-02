use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use chat_shared::packet::{PacketType, ProcessedPacket};

const SHAREDVEC_TIMEOUT: f32 = 0.1;

pub type SharedVec<T> = Arc<Mutex<VecDeque<T>>>;

pub trait SharedVecTools {
  fn remove_first(&self, packet_type: PacketType) -> Option<ProcessedPacket>;
  fn wait_and_remove_first(&self, packet_type: PacketType, timeout: Duration) -> Result<ProcessedPacket, String>;
}

impl SharedVecTools for SharedVec<ProcessedPacket> {
  fn remove_first(&self, packet_type: PacketType) -> Option<ProcessedPacket> {
    let mut incoming = match self.lock() {
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

  fn wait_and_remove_first(&self, packet_type: PacketType, timeout: Duration) -> Result<ProcessedPacket, String> {
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