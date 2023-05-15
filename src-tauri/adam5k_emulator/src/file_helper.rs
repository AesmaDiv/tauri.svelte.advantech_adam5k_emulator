use std::fs::File;
use std::io::{BufReader, BufRead, Error};

use crate::analog::Analog;
use crate::digital::Digital;

pub struct FileHelper {}

impl FileHelper {
  /// Чтение карты регистров из файла для аналоговых каналов
  pub fn read_analog(analog_map: &str) -> Result<Analog, Error> {
    let input = File::open(analog_map)?;
    let buffer = BufReader::new(input);
    let mut result: Vec<u8> = Vec::new();

    for line in buffer.lines() {
      match line {
        Ok(line) => {
          if !line.starts_with("slot") { continue }
          let mut values = line.split(" ")
          .collect::<Vec<&str>>()[1..]
          .iter()
          .map(|s|
            u8::from_str_radix(
              &s.to_string().replace("0x", ""),
              16
            ).unwrap_or(0u8)
          )
          .collect::<Vec<u8>>();
          result.append(&mut values);
        }
        Err(err) => println!("Error reading file: {err}")
      }
    }

    Ok(Analog { channels: result })
  }

  /// Чтение карты регистров из файла для дискретных каналов
  pub fn read_digital(digital_map: &str) -> Result<Digital, Error> {
    let input = File::open(digital_map)?;
    let buffer = BufReader::new(input);
    let mut bitstring: String = String::new();

    for line in buffer.lines() {
      match line {
        Ok(line) => {
          if !line.starts_with("slot") { continue }
          let bytestring = &line.split(" ")
          .collect::<Vec<&str>>()[1..]
          .join("");
          bitstring.push_str(&bytestring);
        }
        Err(err) => println!("Error reading file: {err}")
      }
    }
    bitstring = bitstring.chars().rev().collect();
    let result = u128::from_str_radix(&bitstring, 2).unwrap_or(0u128);

    Ok(Digital { channels: result })
  }

}