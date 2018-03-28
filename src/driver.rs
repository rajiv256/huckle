use alloc::boxed::Box;
use collections::Vec;
use io::{Reader,Writer} ;


pub trait Driver {
  fn init(&mut self);
  fn listen(&mut self);
}

pub trait DriverManager {

  fn get_drivers(&mut self) -> Vec<Box<NetworkDriver + 'static>> ;

}

pub trait NetworkDriver: Driver
{
  fn address(&mut self) -> [u8; 6];

  fn put_frame(&mut self, buf: &[u8]) -> Result<usize,u32> ;

  fn nic_interrupt_handler(&mut self) ; 

}

pub fn adap_ref<T: NetworkDriver + ?Sized>(t: &mut T) -> &mut Adaptor<T> {
  unsafe { ::core::mem::transmute(t) }
}

pub struct Adaptor<T: NetworkDriver + ?Sized>(T);

impl<T: NetworkDriver + ?Sized> Writer for Adaptor<T>
{
  type Err = u32 ;
  fn write(&mut self, buf: &[u8]) -> Result<usize,u32> {
    self.0.put_frame(buf)
  }
}
