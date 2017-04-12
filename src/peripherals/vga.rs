use core::cell::UnsafeCell;


// TODO(john) make these a bit less hard coded
pub const X_MAX: usize = 80;
pub const Y_MAX: usize = 24;

pub type Buffer = [[Entry; X_MAX]; Y_MAX];

extern {
  #[link_name = "vga_buffer"]
  pub static mut GLOBAL: UnsafeCell<Buffer>;
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
#[repr(u8)]
pub enum Color {
  Black      = 0,
  Blue       = 1,
  Green      = 2,
  Cyan       = 3,
  Red        = 4,
  Pink       = 5,
  Brown      = 6,
  LightGray  = 7,
  DarkGray   = 8,
  LightBlue  = 9,
  LightGreen = 10,
  LightCyan  = 11,
  LightRed   = 12,
  LightPink  = 13,
  Yellow     = 14,
  White      = 15,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Entry(u16);

//impl Copy for Entry { }

impl Entry
{
  pub fn new(character: u8, foreground: Color, background: Color) -> Entry
  {
    let color = ((background as u8) << 4) | (foreground as u8);
    Entry(((color as u16) << 8) | (character as u16))
  }

  pub fn eliminate(Entry(bits) : Entry) -> (u8, Color, Color) {
    use core::mem::transmute;

    (bits as u8,
     unsafe { transmute((bits >> 6) as u8) },
     unsafe { transmute(((bits >> 4) & 0x0f) as u8) })
  }
}
