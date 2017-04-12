use spin;

use peripherals::vga;
use io::Writer ; 

// TODO(john): next line is still breaking abstractions (but I can't
// find a nice way to init it either...)
pub static GLOBAL: spin::Mutex<Terminal> = spin::Mutex::new(Terminal {
  current: Point(0,0),
  vga: 0 as *mut vga::Buffer //&mut vga::GLOBAL.value
});

// Hack, related to that above
unsafe impl Send for Terminal {}

struct Point(usize, usize);

pub struct Terminal {
  current: Point,
  vga:     *mut vga::Buffer
}

impl Terminal
{
  fn get_vga_mut(&mut self) -> &mut vga::Buffer {
    unsafe { &mut *self.vga }
  }

  fn put_char(&mut self, c: u8) {
    if c == '\n' as u8 {
      self.current = Point(0, self.current.1 + 1);
    } else {
      self.get_vga_mut()[self.current.1][self.current.0] =
        vga::Entry::new(c, vga::Color::White, vga::Color::Black);
      self.current.0 += 1;
    }

    // line wrap
    if self.current.0 >= vga::X_MAX {
      self.current.0 = 0;
      self.current.1 += 1;
    }

    if self.current.1 >= vga::Y_MAX {
      self.scroll();
      self.current.1 = vga::Y_MAX - 1;
    }
  }


  fn scroll(&mut self)
  {
    let mut rows = self.get_vga_mut().iter_mut();

    let mut n     = rows.next().unwrap();
    let mut suc_n = rows.next();

    while let Some(b) = suc_n {
      ::core::mem::swap(n, b); // TODO(john) just need to copy b -> a
      n = b;
      suc_n = rows.next();
    }
    unsafe {
      *n = ::core::mem::zeroed(); // last row
    }
  }

  pub fn clear_screen(&mut self) {
    unsafe {
      *self.get_vga_mut() = ::core::mem::zeroed();
    }
  }
}

pub fn init_global() {
  let mut guard = GLOBAL.lock();
  unsafe {
    guard.vga = vga::GLOBAL.get();
  }
  guard.clear_screen();
}

impl Writer for Terminal
{
  type Err = u32;

  fn write(&mut self, buf: &[u8]) -> Result<usize, u32> {
    for &c in buf.iter() {
      self.put_char(c);
    }
    Ok(buf.len())
  }
}
