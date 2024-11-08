mod words;
mod addr;

use std::net::UdpSocket;
use std::time::Duration;
use clap::Parser;

use rand::random;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use crate::addr::Addr;

#[derive(Parser)]
#[command(
  name = "Repl", 
  about = "<COMMAND> <ARGS>",
)]
enum Repl {
  /// Connect to another client
   Connect(ConnectArgs),
}

#[derive(clap::Args)]
struct ConnectArgs {
  dst: String,
}

const STUN_SERVER: &str = "stun.l.google.com:19302";
const MAGIC_COOKIE: u32 = 0x2112A442;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  // let args = Cli::parse();

  let socket = UdpSocket::bind("0.0.0.0:0")?;
  socket.set_read_timeout(Some(Duration::from_secs_f32(3.0)))?;

  let transaction_id = random::<[u8; 12]>();
  let request = stun_request(1, 0, transaction_id);


  socket.send_to(&request, STUN_SERVER)?;

  let addr = parse_stun(socket, MAGIC_COOKIE.to_be_bytes());
  println!("your code: {}", addr.as_sequence());

  let mut rl = DefaultEditor::new().expect("unsupported terminal");
  
  
  loop {
    let readline = rl.readline(">> ");
    match readline {
      Ok(line) => {
        let mut line_split: Vec<&str> = vec![">>"];
        let mut commands: Vec<&str> = line.split(' ').collect();
        line_split.append(&mut commands);
        
        rl.add_history_entry(line.as_str());
        
        let parsed = Repl::try_parse_from(line_split);
        
        match parsed {
          Ok(command) => match command {
            Repl::Connect(args) => {
              println!("trying to connect to {}...", args.dst)
            }
          },
          Err(e) => print!("{}", e)
        }
      },
      Err(ReadlineError::Interrupted) => {
        println!("CTRL-C");
        break
      },
      Err(ReadlineError::Eof) => {
        println!("CTRL-D");
        break
      },
      Err(err) => {
        println!("Error: {:?}", err);
        break
      }
    }
  }

  Ok(())
}

fn parse_stun(socket: UdpSocket, tid: [u8; 4]) -> Addr {
  let mut response = [0_u8; 512];
  let _ = socket.recv(&mut response).expect("error receiving response from STUN server");

  let res = &response[26..32];

  Addr {
    addr: [res[2] ^ tid[0], res[3] ^ tid[1], res[4] ^ tid[2], res[5] ^ tid[3]],
    port: [res[0] ^ tid[0], res[1] ^ tid[1]]
  }
}

fn stun_request(ty: u16, length: u16, id: [u8; 12]) -> Vec<u8> {
  let mut retval = Vec::new();

  for byte in ty.to_be_bytes() {
    retval.push(byte);
  }

  for byte in length.to_be_bytes() {
    retval.push(byte);
  }

  // magic cookie
  for byte in 0x2112A442_u32.to_be_bytes() {
    retval.push(byte);
  }

  for byte in id {
    retval.push(byte);
  }

  return retval;
}
