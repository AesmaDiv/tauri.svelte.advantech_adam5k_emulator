pub mod analog;
pub mod digital;
pub mod file_helper;

use analog::Analog;
use digital::Digital;
use file_helper::FileHelper;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Write, Read};
use std::sync::{Arc, Mutex};

/// Интерфейсы карт регистров
pub trait Printable {
  /// Вывод карты регистров в консоль
  fn print(&self);
}
pub trait Registers {
  /// Чтение значений из карты регистров
  fn get_registers(&self, address: usize, count: usize) -> Vec<u8>;
  /// Запись значений в карту регистров
  fn set_registers(&mut self, address: usize, values: &[u8]) -> bool;
}

#[derive (Debug)]
struct AdamData {
  /// карта регистров аналоговых каналов
  pub analog: Mutex<Analog>,
  /// карта регистров дискретных каналов
  pub digital: Mutex<Digital>,
  /// флаг состояния сервера
  pub is_running : Mutex<bool>,
}

#[derive (Debug)]
pub struct AdamEmulator {
  /// внутренние состояния эмулятора
  adam_data: Arc<AdamData>
}

impl AdamEmulator {
  pub fn new() -> Self {
    Self {
      adam_data: Arc::new(AdamData {
        analog: Mutex::new(Analog::new()),
        digital: Mutex::new(Digital::new()),
        is_running: Mutex::new(false),
      })
    }
  }
  pub fn from_maps(analog_map: &str, digital_map: &str) -> Self {
    let analog = FileHelper::read_analog(analog_map);
    let digital = FileHelper::read_digital(digital_map);

    match (analog, digital) {
      (Ok(analog), Ok(digital)) => return Self {
        adam_data: Arc::new(AdamData {
          analog: Mutex::new(analog),
          digital: Mutex::new(digital),
          is_running: Mutex::new(false),
        })
      },
      (Err(err), _) => println!("{:?}", err),
      (_, Err(err)) => println!("{:?}", err),
    }
    AdamEmulator::new()
  }
  pub fn run(&mut self, ip: String) -> bool {
    // запускаем сервер эмулятора и слушаем входящие сообщения
    let ip = ip + ":502";
    match TcpListener::bind(&ip) {
      Ok(listener) => {
        println!("Adam Emulator is listening on {}", ip);
        AdamEmulator::set_running(&self.adam_data, true);
        let adam_data = self.adam_data.clone();
        std::thread::spawn(move || AdamEmulator::handle_listener(listener, &adam_data));

        true
      },
      Err(err) => { 
        println!("Error starting server on {}: {:?}", ip, err);
        AdamEmulator::set_running(&self.adam_data, false);

        false
      }
    }
  }
  fn handle_listener(listener: TcpListener, adam_data: &Arc<AdamData>) {
    for stream in listener.incoming() {
      if !AdamEmulator::get_running(adam_data) {
        println!("Stoping server");
        break;
      }
      match stream {
        Ok(stream) => {
          println!("Incoming stream {:?}", stream);
          let adam_data = adam_data.clone();
          // запускаем поток обработки входящего сообщения
          std::thread::spawn(move || AdamEmulator::handle_stream(stream, adam_data));
        },
        Err(e) => {
          println!("Stream error: {}", e);
        },
      }
    }
    drop(listener);
    println!("Adam Emulator is stopped");
  }
  fn handle_stream(mut stream: TcpStream, adam_data: Arc<AdamData>) {
    let mut buf = [0 as u8; 1024];
    // читаем входящий поток
    while AdamEmulator::get_running(&adam_data) && match stream.read(&mut buf) {
      Ok(size) => {
        // получаем запрос, отбрасывая лишние пустые байты
        let request: Vec<u8> = buf[0..size].to_owned();
        if size == 0 { return }
        if AdamEmulator::check_special_command(&request, &adam_data) { return };
        println!("Command: {:?}", &request);
        // обрабатываем запрос и получаем ответ
        let response = AdamEmulator::get_response(request, &adam_data);
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

  fn check_special_command(request: &Vec<u8>, adam_data: &Arc<AdamData>) -> bool {
    if request.len() == 2 {
      if request[0] == 1 { AdamEmulator::print_map(&adam_data.analog) }
      if request[1] == 1 { AdamEmulator::print_map(&adam_data.digital) }
      if request[0] == 2 { AdamEmulator::set_running(adam_data, false) }
      return true;
    }
    false
  }

  /// Обработка modbus команды
  fn get_response(request: Vec<u8>, adam_data: &Arc<AdamData>) -> Vec<u8> {
    let mut guard_analog = adam_data.analog.lock().unwrap();
    let mut guard_digital = adam_data.digital.lock().unwrap();

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

    result
  }

  /// Формирование ответа
  fn create_answer(request: &[u8], bytes: Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = request[..8].to_vec();
    result.push(bytes.len().try_into().unwrap());
    result.extend(bytes);
    result[5] = result[8] + 3;

    result
  }
  /// Установка флага состояния сервера
  fn set_running(adam_data: &Arc<AdamData>, state: bool) {
    if let Ok(mut guard) = adam_data.is_running.lock() {
      *guard = state;
      drop(guard);
    }
  }
  /// Получение значения флага состояния сервера
  fn get_running(adam_data: &Arc<AdamData>) -> bool {
    let mut result = false;
    if let Ok(guard) = adam_data.is_running.lock() {
      result = *guard;
      drop(guard);
    }
    println!("Server state is {}", result);

    result
  }
  /// Вывод в консоль карты регистров
  fn print_map<T: Printable>(map: &Mutex<T>) {
    if let Ok(guard) = map.lock() {
      guard.print();
      drop(guard);
    }
  }
}