use chat_shared::packet::handshake::HandshakeStatus;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct HandshakeEvent {
  pub status: HandshakeStatus,
  pub sender: String,
}