use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ThreadManager {
  
}

impl ThreadManager {
  pub fn listen(listener: Arc<Mutex<TcpListener>>, streams: Arc<Mutex<Vec<Option<TcpStream>>>>) -> thread::JoinHandle<()> {
    let handle = thread::spawn(move || {
      println!("Waiting for connections...");
      let listener = listener.lock().expect("Could not lock listener");
      for stream_opt in listener.incoming() {
        println!("{:?}", stream_opt);
        // match stream_opt {
        //   Ok(stream) => {
        //     streams
        //       .lock().expect("Could not lock stream list")
        //       .push(Some(stream));
        //   }
        //   Err(_) => {}
        // }
      }
    });
    
    return handle;
  }
}