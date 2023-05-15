pub mod analog;
pub mod digital;
pub mod file_helper;

use analog::Analog;
use digital::Digital;
use file_helper::FileHelper;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Write, Read};
use std::sync::{Arc, Mutex};


#[derive (Debug)]
pub struct AdamEmulator {
  pub analog: Arc<Mutex<Analog>>,
  pub digital: Arc<Mutex<Digital>>,
  pub is_running : bool,
}

impl AdamEmulator {
  pub fn new() -> Self {
    Self {
      analog: Arc::new(Mutex::new(Analog::new())),
      digital: Arc::new(Mutex::new(Digital::new())),
      is_running: false,
    }
  }
  pub fn from_maps(analog_map: &str, digital_map: &str) -> Self {
    let analog = FileHelper::read_analog(analog_map);
    let digital = FileHelper::read_digital(digital_map);

    match (analog, digital) {
      (Ok(analog), Ok(digital)) => return Self {
        analog: Arc::new(Mutex::new(analog)),
        digital: Arc::new(Mutex::new(digital)),
        is_running: false,
      },
      (Err(err), _) => println!("{:?}", err),
      (_, Err(err)) => println!("{:?}", err),
    }
    AdamEmulator::new()
  }
  pub fn run(&mut self, ip: String) {
    // запускаем сервер эмулятора и слушаем входящие сообщения
    self.is_running = true;
    match TcpListener::bind(&ip) {
      Ok(listener) => {
        println!("Adam Emulator is listening on {}", ip);
        for stream in listener.incoming() {
          match stream {
            Ok(stream) => {
              println!("Incoming stream {:?}", stream);
              // запускаем поток обработки входящего сообщения
              let analog = self.analog.clone();
              let digital = self.digital.clone();
              std::thread::spawn(move ||
                AdamEmulator::handle_clent(
                  stream,
                  analog,
                  digital
                )
              );
              if !self.is_running { break }
            },
            Err(e) => {
              println!("Stream error: {}", e);
            },
          }
        }
        drop(listener);
        println!("Adam Emulator is stopped");
      },
      Err(err) => println!("Error starting server: {:?}", err)
    }
  }
  fn handle_clent(mut stream: TcpStream, analog: Arc<Mutex<Analog>>, digital: Arc<Mutex<Digital>>) {
    let mut buf = [0 as u8; 1024];
    // читаем входящий поток
    while match stream.read(&mut buf) {
      Ok(size) => {
        // получаем запрос, отбрасывая лишние пустые байты
        let request: Vec<u8> = buf[0..size].to_owned();
        if request.len() == 0 { return; }
        println!("Command: {:?}", &request);
        // обрабатываем запрос и получаем ответ
        let response = AdamEmulator::get_response(request, analog.clone(), digital.clone());
        if response.len() == 0 { return; }
        println!("Answer: {:?}", &response);
        // отправляем ответ клиенту
        stream.write_all(&response).unwrap();
        true
      },
      Err(_) => {
        println!("Error occured, terminating connection");
        // выключаем подключения
        stream.shutdown(Shutdown::Both).unwrap();
        false
      }
    } {}
    println!("Stream droped");
    drop(stream);
  }

  /// Обработка modbus команды
  fn get_response(request: Vec<u8>, analog: Arc<Mutex<Analog>>, digital: Arc<Mutex<Digital>>) -> Vec<u8> {
    let mut guard_analog = analog.lock().unwrap();
    let mut guard_digital = digital.lock().unwrap();
    if request.len() < 11 {
      if request[0] == 1 { guard_analog.print() }
      if request[1] == 1 { guard_digital.print() }
      drop(guard_analog);
      drop(guard_digital);
      return Vec::new(); 
    }
    // разбор заголовка запроса [модбас, команда, адрес-регистра, кол-во-регистров/значение]
    let address: usize = usize::from(u16::from_be_bytes(request[8..10].try_into().unwrap()));
    let value: usize = usize::from(u16::from_be_bytes(request[10..12].try_into().unwrap()));

    let result: Vec<u8>;
    match request[7] {
      // обрабатываем ЧТЕНИЕ одного или нескольких дискретных каналов (вход или выход)
      1 | 2 => {
        let bytes = guard_digital.get_registers(address, value);
        result = AdamEmulator::create_answer(&request, bytes);
      },
      // обрабатываем ЧТЕНИЕ Одного или нескольких значений аналоговых каналов
      3 | 4 => {
        let bytes = guard_analog.get_registers(address, value);
        result = AdamEmulator::create_answer(&request, bytes);
      },
      // обрабатывает ЗАПИСЬ состояния дискретного выхода
      5 => {
        guard_digital.set_coil(address, value > 0);
        result = request;
      },
      // обрабатываем ЗАПИСЬ значения аналогового выхода
      6 => {
        guard_analog.set_value(address, value as u16);
        result = request;
      }
      // в остальных случаях
      _ => result = request,
    }
    drop(guard_analog);
    drop(guard_digital);

    return  result;
  }

  /// Формирование ответа
  fn create_answer(request: &[u8], bytes: Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = request[..8].to_vec();
    result.push(bytes.len().try_into().unwrap());
    result.extend(bytes);
    result[5] = result[8] + 3;

    return result;
  }
}