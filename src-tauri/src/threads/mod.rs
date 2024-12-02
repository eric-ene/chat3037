use crate::appstate::context::Keys;
use crate::appstate::session;
use crate::helpers::events::{ConnectedPayload, HandshakeEvent, MessagePayload};
use crate::network::stream::StreamThreadTools;
use chat_shared::packet::assign::AssignRequestPacket;
use chat_shared::packet::handshake::HandshakeStatus;
use chat_shared::packet::{Packet, ProcessedPacket};
use chat_shared::stream::read::ReadExact;
use chat_shared::stream::write::SharedWrite;
use eric_aes::{generate_key, aestools};
use eric_aes::rsatools;
use rsa::traits::{PrivateKeyParts, PublicKeyParts};
use rsa::RsaPrivateKey;
use std::collections::VecDeque;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::string::FromUtf8Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{sleep, JoinHandle};
use std::time::Duration;
use eric_aes::aestools::CryptError;
use tauri::{AppHandle, Emitter};

pub fn connect_init(ip: &'static str, stream_arc: session::StreamType) -> JoinHandle<()> {
  return thread::spawn(move || {
    // connect and return stream
    let stream = loop {
      let stream = TcpStream::connect(ip);
      match stream {
        Ok(stream) => break stream,
        Err(e) => {
          println!("Error connecting to server: {:?}", e);
          sleep(Duration::from_millis(100));
          continue;
        }
      }
    };
    println!("connected to server.");

    // get lock on mutex
    let mut guard = stream_arc.lock().unwrap_or_else(|e| {
      println!("Error locking mutex: {:?}", e);
      e.into_inner()
    });

    println!("stream assigned.");

    stream.set_nonblocking(true).unwrap();
    *guard = Some(Arc::new(Mutex::new(stream)));
  });
}

pub fn start_listener(
  app: AppHandle,
  mut stream: session::StreamType,
  incoming: Arc<Mutex<VecDeque<ProcessedPacket>>>,
  keys: Arc<Mutex<Keys>>
) -> JoinHandle<()> {
  return thread::spawn(move || {
    let mut rng = rand::thread_rng();;
    let private_key = RsaPrivateKey::new(&mut rng, 1024).expect("couldn't generate keys!");

    let stream = stream.wait_for();

    let request_packet = AssignRequestPacket {
      e: private_key.e().to_bytes_be(),
      n: private_key.n().to_bytes_be(),
    };

    let request_raw = ProcessedPacket::new_raw(ProcessedPacket::AssignRequest(request_packet));

    loop {
      match stream.write_all_shared(&request_raw) {
        Ok(_) => break,
        Err(e) => {
          println!("couldn't send packet! {}", e);
          continue;
        }
      }
    }

    loop {
      let mut incoming = match incoming.lock() {
        Ok(mut guard) => guard,
        Err(e) => return
      };

      let buf = match stream.read_packet() {
        Ok(vec) => vec,
        Err(e) => { println!("Couldn't read packet! {}", e); continue }
      };

      let packet = Packet::from_rsa_bytes(&buf, &private_key.d().to_bytes_be(), &private_key.n().to_bytes_be());

      let processed = match packet.process() {
        Ok(val) => val,
        Err(e) => {
          println!("couldn't process packet: {}", e);
          return;
        }
      };

      match processed {
        ProcessedPacket::Assign(assign) => {
          let mut keys = match keys.lock() {
            Ok(guard) => guard,
            Err(e) => { println!("Couldn't lock keys! {}", e); return; }
          };

          keys.server_key = Some(assign.aes_key.clone());

          // TODO: Emit an event here if there's time.

          incoming.push_back(ProcessedPacket::Assign(assign));
          break;
        }
        _ => println!("Received non-naming packet too early!")
      }
    }

    // main event loop
    loop {
      let mut buf = match stream.read_packet() {
        Ok(vec) => vec,
        Err(e) => { println!("Couldn't read packet! {}", e); continue }
      };

      let keys_guard = match keys.lock() {
        Ok(guard) => guard,
        Err(e) => {  { println!("Couldn't lock keys! {}", e); return; } }
      };

      let key = match &keys_guard.server_key {
        Some(key) => key,
        None => {
          println!("No server key!"); continue;
        }
      };

      let packet = match Packet::from_aes_bytes(&mut buf, &key) {
        Ok(packet) => packet,
        Err(e) => {
          println!("Couldn't decrypt! Skipping. {:?}", e);
          continue;
        }
      };

      drop(keys_guard);

      let mut incoming = match incoming.lock() {
        Ok(mut guard) => guard,
        Err(e) => return
      };

      let processed = match packet.process() {
        Ok(val) => val,
        Err(e) => {
          println!("couldn't process packet: {}", e);
          return;
        }
      };

      // println!("received packet {:?}", processed);

      match processed {
        ProcessedPacket::Handshake(pack) => match pack.status {
          HandshakeStatus::Request => { // RECIPIENT - GENERATES KEY
            let src = pack.src;
            let id = src.clone().id.unwrap_or(String::from("NO USER ID"));

            let key = generate_key();
            let key_ciphertext = rsatools::encrypt_key(&key, &pack.e, &pack.n);

            let mut keys_lock = match keys.lock() {
              Ok(guard) => guard,
              Err(e) => {
                println!("Can't lock keys! Ignoring packet! {}", e);
                continue;
              }
            };

            keys_lock.req_key = Some(key);
            keys_lock.req_cipher = Some(key_ciphertext);
            drop(keys_lock);

            let payload = HandshakeEvent {
              status: HandshakeStatus::Request,
              sender: format!("{}", src),
              id
            };

            let _ = app.emit("handshake", payload);
          }
          HandshakeStatus::Accept => { // INITIATOR
            let mut guard = match keys.lock() {
              Ok(guard) => guard,
              Err(e) => { println!("Can't lock keys! Ignoring acceptance packet"); continue; }
            };

            let rsa_key = match &guard.req_private {
              Some(val) => val,
              None => { println!("No private key registered. Ignoring acceptance packet"); continue; }
            };

            let aes_key = rsatools::decrpyt_key(&pack.aes_key, &rsa_key.d().to_bytes_be(), &rsa_key.n().to_bytes_be());

            guard.chat_key = Some(aes_key);

            match app.emit("handshake-accepted", {}) {
              Ok(_) => (),
              Err(e) =>  println!("couldn't emit event! {}", e),
            }
          }
          _ => incoming.push_back(ProcessedPacket::Handshake(pack))
        }
        ProcessedPacket::Message(pack) => {
          let guard = match keys.lock() {
            Ok(guard) => guard,
            Err(e) => { println!("Can't lock keys! Ignoring acceptance packet"); continue; }
          };
          
          let key = match &guard.chat_key {
            Some(val) => val,
            None => { println!("No chat key registered. Ignoring message packet"); continue; }
          };
          
          let decrypted = match aestools::decrypt(key, pack.content) {
            Ok(bytes) => bytes,
            Err(e) => { println!("Couldn't decrypt chat message! {:?}", e); continue; }
          };
          
          drop(guard);
          
          let plaintext = match String::from_utf8(decrypted) {
            Ok(text) => text,
            Err(e) => { println!("Mangled message! {}", e); continue; }
          };
          
          let _ = app.emit("incoming-message", MessagePayload {
            content: plaintext
          });
        }
        pack => incoming.push_back(pack)
      };
    }
  });
}