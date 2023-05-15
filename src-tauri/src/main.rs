// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::net::TcpStream;
use std::io::{Read, Write};
use adam5k_emulator::AdamEmulator;


// const IP: &str = "127.0.0.1:502";
// lazy_static::lazy_static! {
//   static ref EMULATOR: Mutex::<AdamEmulator> = Mutex::new(AdamEmulator::new());
// }

#[derive (Debug, serde::Serialize, serde::Deserialize)]
struct ModbusCommand {
  bytes: Vec<u8>
}

fn main() {
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![
    run_server,
    send_request,
  ])
  .run(tauri::generate_context!())
  .expect("error while running tauri application");
}

#[tauri::command]
fn run_server(ip: &str) {
  let ip_str = String::from(ip);
  std::thread::spawn(move || AdamEmulator::from_maps(
    "./adam5k_emulator/src/map/analog.map",
    "./adam5k_emulator/src/map/digital.map"
  ).run(ip_str));
}

#[tauri::command]
fn send_request(ip: &str, bytes: Vec<u8>) -> Vec<u8> {
  // let bytes = json_to_command(bytes);
  match TcpStream::connect(ip) {
    Ok(mut client) => {
      println!("Connect established");
      if let Ok(sent) = client.write(&bytes) {
        println!("{} bytes writen", sent);
        let mut buf = [0u8; 1024];
        if let Ok(received) = client.read(&mut buf) {
          println!("{} bytes read", received);
          let result = buf[..received].to_vec();
          drop(client);
          return result;
        } else { println!("Error receiving answer to respond."); }
      } else { println!("Error sending respond to emulator."); }
    },
    Err(err) => println!("Error connecting emulator: {}", err)
  }
  Vec::new()
}
