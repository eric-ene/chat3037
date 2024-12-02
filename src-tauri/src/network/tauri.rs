use crate::appstate::context::Context;
use crate::helpers::shared_tools::SharedVecTools;
use chat_shared::packet::assign::{NameRequestPacket, NameResponse};
use chat_shared::packet::handshake::{HandshakePacket, HandshakeStatus};
use chat_shared::packet::{PacketType, ProcessedPacket};
use chat_shared::user::User;
use rand::thread_rng;
use rsa::traits::PublicKeyParts;
use rsa::RsaPrivateKey;
use std::io::Write;
use std::string::ToString;
use std::sync::{Arc, LockResult, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};
use chat_shared::packet::encrypt::EncryptedPacket;
use chat_shared::packet::message::MessagePacket;
use eric_aes::aestools;
use eric_aes::aestools::CryptError;
use serde::Deserialize;
use tauri::{Listener, State};
use tokio::time::timeout;

#[derive(Debug, Deserialize)]
pub struct Message {
  pub id: u32,
  pub sender: String,
  pub dst: String,
  pub content: String
}

#[tauri::command]
pub async fn send_message(state: State<'_, Mutex<Context>>, message: Message) -> Result<(), String> {
  let mut ctx = match state.lock() {
    Ok(guard) => guard,
    Err(e) => return Err(e.to_string()),
  };

  let keys_guard = match ctx.keys.lock() {
    Ok(guard) => guard,
    Err(e) => return Err(e.to_string())
  };

  let chat_key = match &keys_guard.chat_key {
    Some(key) => key,
    None => return Err("No server key!".to_string())
  };

  let plaintext = message.content.as_bytes().to_vec();
  let dst = message.dst;
  
  let ciphertext = match aestools::encrypt(&chat_key, plaintext) {
    Ok(bytes) => bytes,
    Err(e) => return Err(format!("{:?}", e))
  };
  
  let msg_packet = MessagePacket {
    receiver: dst,
    content: ciphertext,
  };

  let server_key = match &keys_guard.server_key {
    Some(key) => key,
    None => return Err("No server key!".to_string())
  };

  let packet = match ProcessedPacket::new_raw_aes(ProcessedPacket::Message(msg_packet), &server_key) {
    Ok(pack) => pack,
    Err(e) => return Err(format!("{:?}", e))
  };
  
  drop(keys_guard);
  
  println!("Sent packet.");
  
  return ctx.send_packet(&packet);
}

#[tauri::command]
pub async fn get_identifier(state: State<'_, Mutex<Context>>) -> Result<String, String> {
  let mut ctx = match state.lock() {
    Ok(guard) => guard,
    Err(e) => return Err(e.to_string()),
  };

  let mut retval: Result<String, String> = Err(
    "No name packet found. This is likely becuase \
    there are no packets (server is down)\n or \
    because the client is asking for a name multiple times.".to_string()
  );

  let packet = match ctx.session.remove_first(PacketType::NameAssign) {
    Some(packet) => packet,
    None => return retval,
  };

  let packet = match packet {
    ProcessedPacket::Assign(inner) => inner,
    _ => return Err("Packet doesn't match!".to_string()),
  };

  ctx.id = Some(packet.content.clone());
  return Ok(packet.content);
}

#[tauri::command]
pub async fn request_name(state: State<'_, Mutex<Context>>, name: String) -> Result<(), String> {
  let mut ctx = match state.lock() {
    Ok(guard) => guard,
    Err(e) => return Err(format!("Couldn't lock state mutex: {}", e))
  };

  let id = match &ctx.id {
    Some(val) => val.clone(),
    None => return Err("Wait for the server to assign you an ID!".to_string())
  };

  let request_packet = NameRequestPacket {
    sender: id,
    content: name.clone()
  };

  let keys_guard = match ctx.keys.lock() {
    Ok(guard) => guard,
    Err(e) => return Err(e.to_string())
  };

  let key = match &keys_guard.server_key {
    Some(key) => key,
    None => return Err("No server key!".to_string())
  };

  let packet = match ProcessedPacket::new_raw_aes(ProcessedPacket::NameRequest(request_packet), &key) {
    Ok(pack) => pack,
    Err(e) => return Err(format!("{:?}", e))
  };

  drop(keys_guard);

  let write_result = match ctx.send_packet(&packet) {
    Ok(_) => Ok(()),
    Err(e) => Err(format!("Couldn't write to stream: {}", e)),
  };

  if write_result.is_err() {
    return write_result;
  }

  let packet = match ctx.session.wait_and_remove_first(PacketType::NameResponse, Duration::from_secs_f32(10.0)) {
    Ok(packet) => packet,
    Err(e) => return Err(e)
  };

  let packet = match packet {
    ProcessedPacket::NameResponse(packet) => packet,
    _ => return Err("Packet doesn't match!".to_string()),
  };

  return match packet.status {
    NameResponse::Success => {
      ctx.name = Some(name.clone());
      Ok(())
    },
    NameResponse::Failure(e) => Err(e)
  }
}

#[tauri::command]
/// INITIATOR - REQUESTS AN AES KEY
pub async fn try_connect(state: State<'_, Mutex<Context>>, dst: String) -> Result<String, String> {
  let ctx = match state.lock() {
    Ok(guard) => guard,
    Err(e) => return Err(format!("Couldn't lock state mutex: {}", e))
  };

  let id = match &ctx.id {
    Some(id) => id.clone(),
    None => return Err("Wait for the server to assign you an ID!".to_string())
  };

  let src = User {
    id: Some(id),
    name: ctx.name.clone()
  };

  let rsa_key = RsaPrivateKey::new(&mut thread_rng(), 1024).map_err(|e| e.to_string())?;
  let e = rsa_key.e().to_bytes_be();
  let n = rsa_key.n().to_bytes_be();

  let handshake = HandshakePacket {
    status: HandshakeStatus::Request,
    src,
    dst: dst.clone(),
    e: e.clone(),
    n: n.clone(),
    aes_key: vec![]
  };

  let mut keys_lock = match ctx.keys.lock() {
    Ok(guard) => guard,
    Err(e) => return Err(e.to_string())
  };

  keys_lock.req_private = Some(rsa_key);

  let aes_key = match &keys_lock.server_key {
    Some(key) => key,
    None => return Err("No server key!".to_string())
  };

  let packet = match ProcessedPacket::new_raw_aes(ProcessedPacket::Handshake(handshake), &aes_key) {
    Ok(pack) => pack,
    Err(e) => return Err(format!("{:?}", e))
  };

  drop(keys_lock);

  let write_result = ctx.send_packet(&packet);

  if write_result.is_err() {
    return Err(write_result.expect_err("We literally just checked is_err(). How the hell is this panicking?"));
  }

  let app = ctx.app.clone();

  drop(ctx);

  let timeout = Duration::from_secs_f32(15.0);
  let received: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
  let rec_clone = received.clone();

  let id = app.listen("handshake-accepted", move |evt| {
    match rec_clone.lock() {
      Ok(mut guard) => *guard = true,
      Err(e) => return
    }
  });

  let start = Instant::now();
  while start.elapsed() < timeout {
    let flag = match received.lock() {
      Ok(guard) => guard,
      Err(e) => return Err(e.to_string())
    };

    if *flag {
      println!("acceptance received!");
      return Ok(dst.clone());
    }

    drop(flag);
    sleep(Duration::from_millis(50));
  };

  return Err(format!("Timed out after {:?}", timeout));
}

/// RECIPIENT
#[tauri::command]
pub async fn handle_request(state: State<'_, Mutex<Context>>, dst: String, accept: bool) -> Result<(), String> {
  let ctx = match state.lock() {
    Ok(guard) => guard,
    Err(e) => return Err(format!("Couldn't lock state mutex: {}", e))
  };

  let (status, key_cipher) = match accept {
    true => {
      let mut keys = match ctx.keys.lock() {
        Ok(guard) => guard,
        Err(e) => return Err(e.to_string())
      };

      keys.chat_key = match &keys.req_key {
        Some(key) => Some(key.clone()),
        None => return Err("Request key doesn't exist!".to_string())
      };


      let key_cipher = match &keys.req_cipher {
        Some(ciphertext) => ciphertext.clone(),
        None => return Err("Request cipher doesn't exist!".to_string())
      };


      (HandshakeStatus::Accept, key_cipher)
    },
    false => (HandshakeStatus::Deny, vec![])
  };

  let me = User {
    id: ctx.id.clone(),
    name: ctx.id.clone(),
  };

  let handshake_packet = HandshakePacket {
    status,
    src: me,
    dst,
    aes_key: key_cipher,
    e: vec![],
    n: vec![]
  };

  let mut keys_lock = match ctx.keys.lock() {
    Ok(guard) => guard,
    Err(e) => return Err(e.to_string())
  };

  let aes_key = match &keys_lock.server_key {
    Some(key) => key,
    None => return Err("No server key!".to_string())
  };

  let packet = match ProcessedPacket::new_raw_aes(ProcessedPacket::Handshake(handshake_packet), &aes_key) {
    Ok(pack) => pack,
    Err(e) => return Err(format!("{:?}", e))
  };

  match ctx.send_packet(&packet) {
    Ok(_) => println!("packet sent successfully"),
    Err(e) => return Err(e)
  }

  return match accept {
    true => Ok(()),
    false => Err("Rejected".to_string())
  };
}