use std::io::Write;
use std::sync::{Arc, Mutex};
use chat_shared::stream::write::SharedWrite;
use rsa::RsaPrivateKey;
use tauri::AppHandle;
use crate::appstate::session::Session;
use crate::network::stream::StreamThreadTools;

pub struct Context {
  pub app: AppHandle,
  pub id: Option<String>,
  pub name: Option<String>,
  pub keys: Arc<Mutex<Keys>>,
  pub session: Session,
}

pub struct Keys {
  pub server_key: Option<Vec<u8>>,
  pub chat_key: Option<Vec<u8>>,
  pub req_key: Option<Vec<u8>>,
  pub req_cipher: Option<Vec<u8>>,
  pub req_private: Option<RsaPrivateKey>
}

impl Context {
  pub fn send_packet(&self, packet: &Vec<u8>) -> Result<(), String> {
    let stream = self.session.stream.wait_for();
    return stream.write_all_shared(&packet);
  }
}