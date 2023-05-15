fn main() {
  println!("This is Adam Emulator {}", u32::BITS);
  let analog_map = "./src/map/analog.map";
  let digital_map = "./src/map/digital.map";
    
  let mut emulator = adam5k_emulator::AdamEmulator::from_maps(analog_map, digital_map);
  // let analog = emulator.analog.get_registers(4, 2);
  // println!("{:?}", analog);
  // emulator.analog.set_slot_channel(0, 1, 0x3434);
  // emulator.analog.print("ANALOG ");

  let digital = emulator.digital.lock().unwrap().get_registers(34,8);
  println!("{:?}", digital);
  // emulator.digital.print();
  // emulator.digital.set_registers(120, &[85]);
  emulator.digital.lock().unwrap().set_coil(127, true);
  // emulator.digital.print();
  // emulator.analog.print();

}
