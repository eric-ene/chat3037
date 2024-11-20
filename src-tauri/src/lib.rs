use std::collections::VecDeque;
use crate::appstate::context::Context;
use crate::appstate::session::Session;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use tauri::Manager;

mod appstate;
mod helpers;
mod threads;
mod network;

const SERVER_ADDR: &str = "chat.ericalexander.ca:8081";

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let mut handles = Vec::new();
  
  let incoming = Arc::new(Mutex::new(VecDeque::new()));
  let stream = Arc::new(Mutex::new(None));
  
  let handle = threads::connect_init(SERVER_ADDR, stream.clone());
  handles.push(handle);
  
  sleep(Duration::from_secs_f32(0.1));
  let handle = threads::start_listener(stream.clone(), incoming.clone());
  handles.push(handle);
  
  tauri::Builder::default()
    .setup(
      move |app| {
        app.manage(
          Mutex::new(
            Context {
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
      network::tauri::get_identifier,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  
  for handle in handles {
    let _ = handle.join();
  }
}
