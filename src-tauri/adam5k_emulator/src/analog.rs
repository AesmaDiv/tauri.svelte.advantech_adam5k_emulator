use crate::{Printable, Registers};

#[derive (Debug)]
/// Карта регистров аналоговых каналов
pub struct Analog {
  pub channels: Vec<u8>,
}

impl Analog {
  /// Конструктор
  pub fn new() -> Self {
    Self {
      channels: vec![0u8; 128]
    }
  }
  /// Запись значения в карту по адресу
  pub fn set_value(&mut self, address: usize, value: u16) -> bool {
    self.set_registers(address, &value.to_be_bytes())
  }
  /// Запись значения в карту по номерам слота и канала
  pub fn set_slot_channel(&mut self, slot: usize, channel: usize, value: u16) -> bool {
    self.set_value(slot * 8 + channel, value)
  }
}

impl Registers for Analog {
  fn get_registers(&self, address: usize, count: usize) -> Vec<u8> {
    let start = address * 2;
    let stop = start + count * 2;
    if count == 0 || address > 63 || address + count > 64 {
      println!("Analog:GetRegisters:Error = Wrong parameters");
      return Vec::new();
    }

    self.channels[start..stop].to_vec()
  }
  fn set_registers(&mut self, address: usize, values: &[u8]) -> bool {
    let start = address * 2;
    let stop = start + values.len();
    if values.len() == 0 || stop > 128 {
      println!("Analog:SetRegisters:Error = Wrong parameters");
      return false;
    }
    self.channels[start..stop].copy_from_slice(&values);
    return true;
  }
}

impl Printable for Analog {
  fn print(&self) {
    let mut hex: String = self.channels
    .chunks(16)
    .enumerate()
    .map(|(ind, slot)| format!(
      "\n slot{ind} {}",
      slot
      .chunks(2)
      .map(|pair| format!("{:04X} ", u16::from_be_bytes(pair.try_into().unwrap())))
      .collect::<String>()
    )).collect();
    hex.insert_str(0, " chn0 chn1 chn2 chn3 chn4 chn5 chn6 chn7");
    println!("\nANALOG{hex}");
  }
}
