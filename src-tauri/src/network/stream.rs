use std::net::TcpStream;
use std::sync::TryLockResult;
use std::thread::sleep;
use std::time::Duration;
use crate::appstate::session;
use crate::appstate::session::StreamType;

type StreamReturn = ();
pub trait StreamThreadTools {
  fn block_exec<F, T>(&self, f: F) -> T
    where F: Fn(&mut TcpStream) -> T;
}

impl StreamThreadTools for StreamType {
  /// Blocks the thread until stream_arc is not none, and once stream_arc is some, executes the provided closure with it.
  ///
  /// ### Arguments
  /// * `f` - Closure to execute with the stream
  fn block_exec<F, T>(&self, f: F) -> T
  where
    F: Fn(&mut TcpStream) -> T,
  {
    return loop {
      let mut guard = match self.try_lock() {
        Ok(guard) => guard,
        Err(e) => {
          continue;
        }
      };
      
      match &mut *guard {
        None => {}
        Some(stream) => {
          break f(stream)
        },
      }
    }
    
  }
}
