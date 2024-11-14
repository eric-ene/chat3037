use std::net::{TcpListener, TcpStream, UdpSocket};
use std::sync::Mutex;
use tauri::Manager;
use rand::random;
use crate::appstate::context::Context;
use crate::appstate::session::Session;
use crate::utils::{parse_stun, stun_request};

mod utils;
mod appstate;
mod helpers;
mod data;

const STUN_SERVER: &str = "stun.l.google.com:19302";
pub const MAGIC_COOKIE: u32 = 0x2112A442;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let socket = UdpSocket::bind("0.0.0.0:0").expect("couldn't bind to socket!");

  let transaction_id = random::<[u8; 12]>();
  let request = stun_request(1, 0, transaction_id);

  socket.send_to(&request, STUN_SERVER).expect("couldn't send STUN request!");

  let addr = parse_stun(&socket, MAGIC_COOKIE.to_be_bytes());
  println!("your code: {}", addr.as_sequence());
  
  drop(socket);
  let listener = TcpListener::bind(format!("0.0.0.0:{}", addr.port_str())).unwrap();

  tauri::Builder::default()
    .setup(
      move |app| {
        app.manage(
          Mutex::new(
            Context {
              session: Session {
                src: Some(addr),
                dst: None,
                listener,
                in_stream: None,
                out_stream: None,
              }
            }
          )
        );
        Ok(())
      }
    )
    .plugin(tauri_plugin_shell::init())
    .plugin(tauri_plugin_dialog::init())
    .invoke_handler(tauri::generate_handler![
      utils::generate_identifier,
      helpers::network::try_connect,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
