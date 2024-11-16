use serde::{Deserialize, Serialize};

#[repr(u8)]
pub enum PacketType {
  NameAssign = 0xF0,
}

#[repr(u8)]
pub enum PacketSymbols {
  Eof = 0x04,
}

#[derive(Debug)]
pub struct Packet {
  pub header: Vec<u8>, // THIS SHOULD ALWAYS HAVE A SIZE OF 8
  pub data: Vec<u8>,
}

#[derive(Debug)]
pub enum ProcessedPacket {
  Assign(AssignPacket),
  Message(MessagePacket),
}

#[derive(Debug)]
pub struct AssignPacket {
  pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessagePacket {
  pub sender: String,
  pub receiver: String,
  pub content: String,
}

impl Packet {
  pub fn from_bytes(bytes: Vec<u8>) -> Self {
    let header = bytes[0..8].to_vec();
    let data = bytes[8..bytes.len()-1].to_vec();

    Self {
      header,
      data
    }
  }
  
  pub fn process(&self) -> Result<ProcessedPacket, String> {
    match PacketType::try_from(self.header[0]) {
      Ok(PacketType::NameAssign) => {
        let content = match String::from_utf8(self.data.clone()) {
          Ok(content) => content,
          Err(e) => return Err(e.to_string())
        };
        
        Ok(ProcessedPacket::Assign(AssignPacket { content }))
      }
      Err(e) => Err(e.to_string())?,
    }
  }
}

impl From<PacketType> for u8 {
  fn from(val: PacketType) -> Self {
    val as u8
  }
}

impl TryFrom<u8> for PacketType {
  type Error = String;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      0xF0 => Ok(PacketType::NameAssign),
      val => Err(format!("Invalid packet type: {:#04X}", val)),
    }
  }
}