use std::collections::VecDeque;
use crate::appstate::context::Context;
use crate::appstate::session::Session;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use tauri::Manager;

mod appstate;
mod helpers;
mod threads;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let incoming = Arc::new(Mutex::new(VecDeque::new()));
  
  let handle = threads::connect_and_listen("chat.ericalexander.ca:8081".to_string(), incoming.clone());
  
  tauri::Builder::default()
    .setup(
      move |app| {
        app.manage(
          Mutex::new(
            Context {
              session: Session::new(incoming.clone())
            }
          )
        );
        Ok(())
      }
    )
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_dialog::init())
    .invoke_handler(tauri::generate_handler![
      helpers::network::get_identifier,
      helpers::network::try_connect,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  
  let _ = handle.join();
}
