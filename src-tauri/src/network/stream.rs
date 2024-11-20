use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;
use crate::appstate::session;
use crate::appstate::session::StreamType;

type StreamReturn = ();
pub trait StreamThreadTools {
  fn block_exec<F>(&self, f: F) -> StreamReturn
    where F: Fn(&mut TcpStream) -> StreamReturn;
}

impl StreamThreadTools for StreamType {
  /// Blocks the thread until stream_arc is not none, and once stream_arc is some, executes the provided closure with it.
  ///
  /// ### Arguments
  /// * `f` - Closure to execute with the stream
  fn block_exec<F>(&self, f: F) -> StreamReturn
  where
    F: Fn(&mut TcpStream) -> StreamReturn,
  {
    return loop {
      let mut guard = self.lock().unwrap_or_else(|e| {
        println!("Mutex poisoned! {}", e);
        e.into_inner()
      });

      match &mut *guard {
        None => {}
        Some(stream) => break f(stream),
      }

      sleep(Duration::from_millis(20));
    }
  }
}
