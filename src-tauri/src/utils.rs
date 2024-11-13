use std::net::UdpSocket;
use std::sync::{LockResult, Mutex};
use tauri::State;
use crate::appstate::addr::Addr;
use crate::appstate::context::Context;
use crate::MAGIC_COOKIE;

#[tauri::command]
pub fn generate_identifier(state: State<'_, Mutex<Context>>) -> String {
  return match state.lock() {
    Ok(ctx) => match ctx.session.src {
      Some(val) => val.as_sequence(),
      None => String::from("No host address!")
    }
    Err(_) => String::from("Couldn't lock state mutex!")
  }
}

pub fn parse_stun(socket: &UdpSocket, tid: [u8; 4]) -> Addr {
  let mut response = [0_u8; 512];
  let _ = socket
    .recv(&mut response)
    .expect("error receiving response from STUN server");

  let res = &response[26..32];

  Addr {
    addr: [
      res[2] ^ tid[0],
      res[3] ^ tid[1],
      res[4] ^ tid[2],
      res[5] ^ tid[3],
    ],
    port: [res[0] ^ tid[0], res[1] ^ tid[1]],
  }
}

pub fn stun_request(ty: u16, length: u16, id: [u8; 12]) -> Vec<u8> {
  let mut retval = Vec::new();

  for byte in ty.to_be_bytes() {
    retval.push(byte);
  }

  for byte in length.to_be_bytes() {
    retval.push(byte);
  }

  // magic cookie
  for byte in MAGIC_COOKIE.to_be_bytes() {
    retval.push(byte);
  }

  for byte in id {
    retval.push(byte);
  }

  return retval;
}