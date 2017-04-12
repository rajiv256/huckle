use peripherals::mycpu::Port;

static KEY_CODE_TO_ASCII: [u8; 59] = *b"??1234567890-=??qwertyuiop[]\n?asdfghjkl;'`?\\zxcvbnm,./?*? ?"; 

#[derive(Copy, Clone)]
pub struct Keyboard {
  pub callback:     fn (u8),
  pub control_port: Port,
  pub data_port:    Port
}

bitflags! {
  flags Status: u8 {
    const OUTPUT_FULL     = 0b_00000001,
    const INPUT_FULL      = 0b_00000010,
    const SYSTEM          = 0b_00000100,
    const COMMAND         = 0b_00001000,
    const KEYBOARD_LOCKED = 0b_00010000,
    const AUX_OUTPUT_FULL = 0b_00100000,
    const TIMEOUT         = 0b_01000000,
    const PARITY_ERROR    = 0b_10000000
  }
}

impl Keyboard {
  
  #[allow(dead_code)]
  fn get_status(&mut self) -> Status {
    Status::from_bits(self.control_port.in8()).unwrap()
  }
  
  /*
  fn send_command(&mut self, command: Command) {
    while get_status().output_full as bool {}
    control_port.write_u8(command);
  }*/
  
  pub fn got_interrupted(&self) {
    let keycode = self.data_port.in8();
    match KEY_CODE_TO_ASCII.get(keycode as usize) {
      Some(ascii) => (self.callback)(*ascii),
      None => ()
    }
  }
    
}
