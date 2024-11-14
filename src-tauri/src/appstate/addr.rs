use std::fmt;
use std::fmt::Formatter;
use std::net::{SocketAddr, ToSocketAddrs};
use std::vec::IntoIter;

use crate::data::WORDS;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct Addr {
  pub addr: [u8; 4],
  pub port: [u8; 2],
}


impl Addr {
  pub fn addr(&self) -> SocketAddr {
    return self
      .to_socket_addrs()
      .expect("couldn't resolve addr!")
      .next()
      .expect("couldn't index into addr!");
  }

  pub fn as_sequence(&self) -> String {
    format!(
      "{}-{}-{}-{}-{}-{}",
      WORDS[self.addr[0] as usize],
      WORDS[self.addr[1] as usize],
      WORDS[self.addr[2] as usize],
      WORDS[self.addr[3] as usize],
      WORDS[self.port[0] as usize],
      WORDS[self.port[1] as usize]
    )
  }

  pub fn from_sequence(seq: &String) -> Result<Self, String> {
    let mut addr = [0u8; 4];
    let mut port = [0u8; 2];

    let split: Vec<&str> = seq.split('-').collect();

    if split.len() != 6 {
      return Err("Incorrect number of words in code. Must be 6.".to_string());
    }

    // convert ip
    for i in 0_usize..4 {
      match WORDS.iter().position(|x| x == &split[i]) {
        None => {
          return Err(format!(
            "Word \"{}\" not found in database. Check spelling.",
            &split[i]
          ))
        }
        Some(index) => addr[i] = index as u8,
      }
    }

    // convert port
    for i in 0_usize..2 {
      match WORDS.iter().position(|x| x == &split[i]) {
        None => {
          return Err(format!(
            "Word \"{}\" not found in database. Check spelling.",
            &split[i + 4]
          ))
        }
        Some(index) => port[i] = index as u8,
      }
    }

    Ok(Self { addr, port })
  }

  pub fn port_str(&self) -> String {
    return format!("{}", u16::from_be_bytes(self.port));
  }
}

impl fmt::Display for Addr {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}",
      format!(
        "{}.{}.{}.{}:{}",
        self.addr[0],
        self.addr[1],
        self.addr[2],
        self.addr[3],
        u16::from_be_bytes(self.port)
      )
    )
  }
}

impl ToSocketAddrs for Addr {
  type Iter = IntoIter<SocketAddr>;

  fn to_socket_addrs(&self) -> std::io::Result<Self::Iter> {
    match self.to_string().to_socket_addrs() {
      Ok(iter) =>  Ok(iter),
      Err(e) => Err(e)
    }
  }
}