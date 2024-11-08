use std::fmt;
use std::fmt::Formatter;
use crate::words::WORDS;

pub struct Addr {
  pub addr: [u8; 4],
  pub port: [u8; 2]
}

impl Addr {
  pub fn as_sequence(&self) -> String {
    return format!(
      "{}-{}-{}-{}-{}-{}",
      WORDS[self.addr[0] as usize].to_lowercase(),
      WORDS[self.addr[1] as usize].to_lowercase(),
      WORDS[self.addr[2] as usize].to_lowercase(),
      WORDS[self.addr[3] as usize].to_lowercase(),
      WORDS[self.port[0] as usize].to_lowercase(),
      WORDS[self.port[1] as usize].to_lowercase()
    );
  }
}

impl fmt::Display for Addr {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", format!(
      "{}.{}.{}.{}:{}",
      self.addr[0], self.addr[1], self.addr[2], self.addr[3],
      u16::from_be_bytes(self.port)
    ))
  }
}