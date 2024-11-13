use std::net::UdpSocket;
use crate::appstate::addr::Addr;

pub struct Session {
  pub src: Option<Addr>,
  pub dst: Option<Addr>,
  pub sock: UdpSocket
}

impl Session {
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

  pub fn connect(&self) -> Result<(), String> {
    let dst = match &self.dst {
      None => return Err("No destination!".to_string()),
      Some(val) => val
    };
    
    match self.sock.connect(dst.addr()) {
      Ok(_) => {}
      Err(e) => return Err(e.to_string())
    }

    Ok(())
  }
  
  pub fn send(&self, msg: String) {
    let bytes = msg.as_bytes();


    match self.sock.send(bytes) {
      Ok(n) => println!("sent {} bytes", n),
      Err(e) => println!("{}", e)
    }
  }
  
  pub fn get_other(&self) -> Result<String, String> {
    
    return Err(String::from("Not implemented"))
  }
}
