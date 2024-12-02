use std::net::TcpStream;
use std::sync::{Arc, Mutex, TryLockResult};
use std::thread::sleep;
use std::time::Duration;
use chat_shared::stream::read::SharedStream;
use crate::appstate::session;
use crate::appstate::session::StreamType;

pub trait StreamThreadTools {
  
  fn wait_for(&self) -> SharedStream;
}

impl StreamThreadTools for StreamType {
  fn wait_for(&self) -> SharedStream {
    return loop {
      let mut guard = match self.try_lock() {
        Ok(guard) => guard,
        Err(e) => {
          continue;
        }
      };
      
      match &*guard {
        None => {}
        Some(stream) => {
          break stream.clone();
        },
      }
    }
  }
}
