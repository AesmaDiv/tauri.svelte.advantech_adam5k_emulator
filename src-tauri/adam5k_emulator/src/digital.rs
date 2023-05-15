use std::convert::TryInto;
#[derive (Debug)]
pub struct Digital {
  pub channels: u128,
}

impl Digital {
  pub fn new() -> Self {
    Self { channels: 0u128 }
  }
  /// Получение состояний каналов
  pub fn get_registers(&self, address: usize, count: usize) -> Vec<u8> {
    if count == 0 || address + count > 128 { 
      println!("Digital:GetRegisters:Error = Wrong parameters");
      return Vec::new();
    }

    let lshift = 128 - (address + count);
    let rshift = 128 - count;
    let value = (self.channels << lshift) >> rshift;
    // println!("\n     {}", (0..13).map(|_| "9876543210").collect::<String>().trim_start_matches("98"));
    // println!("FULL {:0128b}", self.channels);
    // println!("BITS {:0128b}\n", value);

    let num_of_bytes = 15 - (count - 1) / 8;
    let result = Vec::from(value.to_ne_bytes())
    .drain(num_of_bytes..)
    .collect();

    return result;
  }

  pub fn set_registers(&mut self, address: usize, values: &[u8]) -> bool {
    let length = values.len();
    if address > 127 || length == 0 || length > 16 { 
      println!("Digital:GetRegisters:Error = Wrong parameters");
      return false
    }
    let mut bytes = vec![0u8; 16 - length];
    bytes.extend(values);
    match bytes.try_into() {
      Ok(bytes) => {
        let bits: u128 = u128::from_ne_bytes(bytes);
        self.switch_bits(address, bits, length * 8);
      },
      Err(err) => println!("Digital:SetRegisters:Error = Fail to convert value {:?}", err)
    }

    return true;
  }

  pub fn set_coil(&mut self, address: usize, value: bool) -> bool {
    if address > 127 { 
      println!("Digital:GetRegisters:Error = Wrong parameters");
      return false;
    }
    self.switch_bits(address, if value { 1u128 } else { 0u128 }, 1);
    return true;
  }

  pub fn set_slot_channel(&mut self, slot: usize, channel: usize, value: bool) -> bool {
    self.set_coil(slot * 16 + channel, value)
  }

  fn switch_bits(&mut self, address: usize, mut bits: u128, bit_len: usize) {
    let mut mask = u128::MAX;
    mask = mask >> (128 - bit_len) << address;
    bits <<= address;

    // println!("\n     {}", (0..13).map(|_| "9876543210").collect::<String>().trim_start_matches("98"));
    // println!("FULL {:0128b}", self.channels);
    // println!("Mask {:0128b}", mask);
    // println!("Bits {:0128b}", bits);
    self.channels = !mask & self.channels | bits;
    // println!("RSLT {:0128b}\n", self.channels);
  }

  pub fn print(&self) {
    let bitstring: String = format!("{:0128b}", self.channels)
    .chars()
    .rev()
    .collect::<String>()
    .as_bytes()
    .chunks(16)
    .enumerate()
    .map(|(ind, slot)| format!(
      "\n  slot{ind} {}",
      slot.to_vec()
      .chunks(8)
      .map(|bits| String::from_utf8(bits.to_vec()).unwrap())
      .collect::<Vec<String>>()
      .join(" ")
    ))
    .collect::<String>();
    println!("\nDIGITAL 01234567{bitstring}");
  }

}