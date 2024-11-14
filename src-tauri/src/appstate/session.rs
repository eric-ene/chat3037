use std::{io::Write, net::{TcpListener, TcpStream, UdpSocket}};
use std::io::{ErrorKind, Read};
use std::string::FromUtf8Error;
use std::time::Duration;
use crate::appstate::addr::Addr;

pub struct Session {
  pub src: Option<Addr>,
  pub dst: Option<Addr>,
  pub listener: TcpListener,
  pub out_stream: Option<TcpStream>,
  pub in_stream: Option<TcpStream>
}

impl Session {
  pub fn new(addr: Addr, listener: TcpListener) -> Self {
    return Self {
      src: Some(addr),
      dst: None,
      listener,
      in_stream: None,
      out_stream: None,
    };
  }
  
  pub fn try_prase(&mut self, dst_code: &String) -> Result<(), String> {
    match Addr::from_sequence(dst_code) {
      Ok(val) => {
        self.dst = Some(val);
        Ok(())
      }
      Err(e) => {
        Err(e)
      }
    }
  }

  pub fn connect(&mut self) -> Result<(), String> {
    let dst = match &self.dst {
      None => return Err("No destination!".to_string()),
      Some(val) => val
    };
    
    self.out_stream = Some(match TcpStream::connect_timeout(&dst.addr(), Duration::from_secs(5)) {
      Ok(stream) => stream,
      Err(e) => return Err(e.to_string())
    });

    Ok(())
  }
  
  pub fn send(&mut self, msg: String) -> Result<usize, std::io::Error> {
    let bytes = msg.as_bytes();

    return match self.out_stream {
      Some(ref mut stream) => stream.write(bytes),
      None => Err(std::io::Error::new(std::io::ErrorKind::NotConnected, "Not Connected!")),
    };
  }

  pub fn accept(&mut self) -> Result<(), std::io::Error> {
    for connection in self.listener.incoming() {
      match connection {
        Ok(stream) => match stream.peer_addr() {
          Ok(addr) => match self.dst {
            Some(dst) => {
              if addr.to_string() == dst.to_string() {
                unimplemented!()
              }
            }
            None => return Err(std::io::Error::new(ErrorKind::NotConnected, "dst not set."))
          }
          Err(e) => return Err(std::io::Error::new(ErrorKind::AddrNotAvailable, "Can't get other address!"))
        }
        Err(_) => continue
      }
    }
    
    return Err(std::io::Error::new(ErrorKind::NotFound, "No incoming connection from dst"));
  }

  pub fn read(&mut self) -> Result<String, std::io::Error> {
    let mut buf = Vec::new();

    let count = match self.in_stream {
      Some(ref mut stream) => match stream.read_to_end(&mut buf) {
        Ok(count) => count,
        Err(e) => return Err(e)
      }
      None => return Err(std::io::Error::new(std::io::ErrorKind::NotConnected, "Not Connected!")),
    };
    
    if count != 0 {
      match String::from_utf8(buf) {
        Ok(s) => return Ok(s),
        Err(e) => return Err(std::io::Error::new(
          std::io::ErrorKind::InvalidData, 
          "Couldn't parse utf8!"
        ))
      }
    }
    
    return Err(std::io::Error::new(
      std::io::ErrorKind::UnexpectedEof, 
      "Read 0 bytes without error?"
    ));
  }
  
  pub fn get_other(&self) -> Result<String, String> {
    return Ok(String::from("Bondo Reems"));
    return Err(String::from("Not implemented"))
  }
}
