use std::net::TcpStream;
use std::sync::{MutexGuard, TryLockError, TryLockResult};
use std::thread::sleep;
use std::time::Duration;
use crate::appstate::session;
use crate::appstate::session::StreamType;

type StreamReturn = ();
pub trait StreamThreadTools {
  fn block_exec<F>(&self, f: F) -> StreamReturn
    where F: Fn(&mut TcpStream) -> StreamReturn;
}

const MUTEX_SLEEP_DUR: Duration = Duration::from_millis(100);

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
      // let mut guard = self.lock().unwrap_or_else(|e| {
      //   println!("Mutex poisoned! {}", e);
      //   e.into_inner()
      // });
      
      let mut guard = match self.try_lock() {
        Ok(guard) => guard,
        Err(e) => match e {
          TryLockError::Poisoned(e) => {
            println!("Mutex poisioned! {}", e.to_string());
            e.into_inner()
          }
          TryLockError::WouldBlock => {
            sleep(MUTEX_SLEEP_DUR);
            continue;
          }
        }
      };

      match &mut *guard {
        None => {}
        Some(stream) => { 
          println!("Stream acquired: {:?}", stream.local_addr().unwrap());
          break f(stream);
        },
      }
      
      sleep(MUTEX_SLEEP_DUR);
    }
  }
}
