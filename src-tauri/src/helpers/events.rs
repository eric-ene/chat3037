use chat_shared::packet::handshake::HandshakeStatus;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize)]
pub struct HandshakeEvent {
  pub status: HandshakeStatus,
  pub sender: String,
  pub id: String,
}

#[derive(Clone, Serialize,  Deserialize)]
pub struct ConnectedPayload {
  pub name: String,
}