use std::collections::VecDeque;
use crate::appstate::context::Context;
use crate::appstate::session::Session;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::Manager;

mod appstate;
mod helpers;
mod threads;
mod network;

const SERVER_ADDR: &str = "chat.ericalexander.ca:8081";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let handles = Arc::new(Mutex::new(Vec::new()));
  let mut hand = handles.lock().unwrap();
  
  let incoming = Arc::new(Mutex::new(VecDeque::new()));
  let stream = Arc::new(Mutex::new(None));
  
  let handle = threads::connect_init(SERVER_ADDR, stream.clone());
  hand.push(handle);
  drop(hand);
  
  let inner_handles = handles.clone();

  tauri::Builder::default()
    .setup(
      move |app| {
        let handle = threads::start_listener(app.handle().clone(), stream.clone(), incoming.clone());
        let mut hand = inner_handles.lock().unwrap();
        hand.push(handle);
        drop(hand);
        
        app.manage(
          Mutex::new(
            Context {
              id: None,
              name: None,
              session: Session::new(stream.clone(), incoming.clone())
            }
          )
        );
        Ok(())
      }
    )
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_dialog::init())
    .invoke_handler(tauri::generate_handler![
      network::tauri::get_identifier,
      network::tauri::request_name,
      network::tauri::try_connect,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  
  let mut hand = handles.lock().unwrap();
  for handle in hand.drain(..) {
    let _ = handle.join();
  }
}
