use std::net::TcpStream;
use crate::appstate::context::Context;
use std::sync::{Arc, LockResult, Mutex, MutexGuard};
use std::thread::sleep;
use std::time::Duration;
use chat_shared::packet::ProcessedPacket;
use tauri::State;
use crate::appstate::session;

#[tauri::command]
pub async fn get_identifier(state: State<'_, Mutex<Context>>) -> Result<String, String> {
  let ctx = match state.lock() {
    Ok(guard) => guard,
    Err(e) => return Err(e.to_string()),
  };

  let mut incoming = match ctx.session.incoming.lock() {
    Ok(guard) => guard,
    Err(e) => return Err(e.to_string()),
  };

  let mut marked = Vec::new();
  let mut retval: Result<String, String> = Err(
    "No name packet found. This is likely becuase \
    there are no packets (server is down)\n or \
    because the client is asking for a name multiple times.".to_string()
  );
  
  for (index, packet) in incoming.iter().enumerate() {
    match packet {
      ProcessedPacket::Assign(p) => {
        // do stuff with the packet
        retval = Ok(p.content.clone());

        // mark packet for removal
        marked.push(index);
      }
      _ => ()
    }
  }

  marked.sort_unstable();
  marked.reverse();

  // remove processed packets
  for index in marked {
    incoming.remove(index);
  }

  return retval;
}

#[tauri::command]
pub fn try_connect(state: State<'_, Mutex<Context>>, seq: String, name: String) -> Result<String, String> {
  let mut ctx = state.lock().expect("couldn't lock state mutex!");

  
  return Err(String::from("Unimplemented"));
}
