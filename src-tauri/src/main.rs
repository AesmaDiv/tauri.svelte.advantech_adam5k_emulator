// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::net::TcpStream;
use std::io::{Read, Write};
use std::time::Duration;
use adam5k_emulator::AdamEmulator;


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
fn run_server(ip: &str) -> bool {
  AdamEmulator::new(//from_maps(
  //   "./adam5k_emulator/src/map/analog.map",
  //   "./adam5k_emulator/src/map/digital.map"
  ).run(ip.to_string())
}

#[tauri::command]
fn send_request(ip: String, bytes: Vec<u8>) -> Vec<u8> {
  if let Ok(addr) = (ip + ":502").parse() {
    match TcpStream::connect_timeout(&addr, Duration::from_millis(100)) {
      Ok(mut client) => {
        println!("Connect established");
        if let Ok(sent) = client.write(&bytes) {
          println!("REQUEST {} bytes writen", sent);
          let mut buf = [0u8; 1024];
          if let Ok(received) = client.read(&mut buf) {
            println!("RESPOND {} bytes read", received);
            let result = buf[..received].to_vec();
            drop(client);
            return result;
          } else { println!("Error receiving answer to respond."); }
        } else { println!("Error sending respond to emulator."); }
      },
      Err(err) => println!("Error connecting emulator: {}", err)
    }
  } else { println!("Unable to parse IP address") }
  Vec::new()
}
