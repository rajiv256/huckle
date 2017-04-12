use core::fmt::Display ; 

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Debug)]
pub struct Port(u16);

impl Port {

  pub const fn new(number: u16) -> Port {
    Port(number)
  }

  pub fn in8(self) -> u8 {
    unsafe { ::cpu::in8(self.0) }
  }

  pub fn out8(self, num: u8) {
    unsafe { ::cpu::out8(self.0, num) }
  }

  pub fn in16(self) -> u16 {
    unsafe { ::cpu::in16(self.0) }
  }

  pub fn out16(self, num: u16) {
    unsafe { ::cpu::out16(self.0, num) }
  }

  pub fn in32(self) -> u32 {
    unsafe { ::cpu::in32(self.0) }
  }

  pub fn out32(self, num: u32) {
    unsafe { ::cpu::out32(self.0, num) }
  }

  pub fn io_wait() {
    Port::new(0x80).out8(0);
  }

}

