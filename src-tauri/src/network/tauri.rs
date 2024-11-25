use crate::appstate::context::Context;
use crate::network::stream::StreamThreadTools;
use chat_shared::packet::assign::{NameRequestPacket, NameResponse};
use chat_shared::packet::handshake::{HandshakePacket, HandshakeStatus};
use chat_shared::packet::{PacketType, ProcessedPacket};
use chat_shared::user::User;
use std::fmt::format;
use std::io::Write;
use std::net::TcpStream;
use std::string::ToString;
use std::sync::{Arc, LockResult, Mutex, MutexGuard};
use std::thread::sleep;
use std::time::Duration;
use tauri::State;

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

  let packet = ProcessedPacket::new_raw(ProcessedPacket::NameRequest(request_packet));

  let write_result =  ctx.session.stream.block_exec::<_, Result<(), String>>(|stream| {
    return match stream.write_all(&packet) {
      Ok(_) => Ok(()),
      Err(e) => Err(format!("Couldn't write to stream: {}", e)),
    }
  });
  
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
pub async fn try_connect(state: State<'_, Mutex<Context>>, dst: String) -> Result<(), String> {
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

  let handshake = HandshakePacket {
    status: HandshakeStatus::Request,
    src,
    dst
  };
  
  let packet = ProcessedPacket::new_raw(ProcessedPacket::Handshake(handshake));
  
  let write_result = ctx.session.stream.block_exec(|stream| {
    return match stream.write_all(&packet) {
      Ok(_) => Ok(()),
      Err(e) => Err(format!("Couldn't write to stream: {}", e)),
    };
  });

  if write_result.is_err() {
    return write_result;
  }

  let packet = match ctx.session.wait_and_remove_first(PacketType::NameResponse, Duration::from_secs_f32(20.0)) {
    Ok(packet) => packet,
    Err(e) => return Err(e)
  };

  let packet = match packet {
    ProcessedPacket::Handshake(packet) => packet,
    _ => return Err("Packet doesn't match!".to_string()),
  };
  
  return match packet.status {
    HandshakeStatus::Accept => Ok(()),
    HandshakeStatus::Deny => Err("The other user denied your request.".to_string()),
    HandshakeStatus::NotFound => Err("User not found.".to_string()),
    _ => Err("Unexpected error!".to_string())
  };
}
