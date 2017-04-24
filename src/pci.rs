use core::mem::{transmute,size_of} ; 
use alloc::boxed::Box;

use collections::Vec;

use peripherals::mycpu::Port;
use rtl8139::Rtl8139;
use driver::{DriverManager, NetworkDriver};
use ::net::NetworkStack ; 


pub struct PciManifest {
  pub register_limit: usize,
  pub device_id: u16,
  pub vendor_id: u16,
  pub bus_master: bool
}

pub struct PortGranter {
  base: usize,
  limit: usize
}

impl PortGranter {

  pub fn get(&self, offset: usize) -> Port {
    if offset as u16 >= self.limit as u16 { // TODO(ryan): doesn't take width into consideration
      panic!();
    }
    let address: u16 = (self.base + offset) as u16; // TODO(ryan): overflow ?
    Port::new(address)
  }

}

pub struct Pci {
  address_port: Port,
  data_port: Port
}

struct PciHeader {
  shared: SharedHeader,
  rest: HeaderType
}

#[allow(dead_code)]
#[repr(packed)]
struct SharedHeader {
  vendor: u16,
  device: u16,
  command: u16,
  status: u16,
  revision: u8,
  prog_if: u8,
  subclass: u8,
  class_code: u8,
  cache_line_size: u8,
  latency_timer: u8,
  header_type: u8,
  bist: u8
}

#[allow(dead_code)]
#[repr(packed)]
struct Header1 {
  base_addresses: [u32; 6],
  cardbus_pointer: u32,
  subsystem_vendor: u16,
  subsystem: u16,
  expansion_rom_address: u32,
  capabilities_pointer: u8,
  reserved: [u8; 7],
  interrupt_line: u8,
  interrupt_pin: u8,
  min_grant: u8,
  max_latency: u8
}

enum HeaderType {
  Basic(Header1),
  Todo,
}

fn read_into<'a, T, S>(slice: &'a [S]) -> Box<T> {
  assert!(size_of::<S>() * slice.len() == size_of::<T>());
  let ret: Box<T> = unsafe { transmute(slice.as_ptr()) };
  return ret
}
impl Pci {

  pub fn new() -> Pci {
    let address_port = Port::new(0xcf8);
    let data_port = Port::new(0xcfc);
    Pci { address_port: address_port, data_port: data_port }
  }

  pub fn init(&mut self) {}

  fn build_address(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
    if (function & 0x03 != 0x00) || (device >= 0x1 << 5) || (function >= 0x1 << 3)  {
      panic!()
    } else {
      return ((0x1 as u32) << 31) | ((bus as u32) << 16) | ((device as u32) << 11) | ((function as u32) << 8) | offset as u32;
    }
  }

  pub fn read(&mut self, bus: u8, device: u8, function: u8, offset: u8) -> Result<u32, ()> {
    let address = Pci::build_address(bus, device, function, offset);
    
    self.address_port.out32(address);
    Port::io_wait();
    let input = self.data_port.in32();
    Ok(input)
  }

  pub fn read_bytes(&mut self, bus: u8, device: u8, start_address: u16, size: u16) -> Vec<u32> {
    assert_eq!(size % 4, 0);
    assert_eq!(start_address % 4, 0);

    let mut v = Vec::new();
    for i in 0_u16..(size / 4) {
      let (offset, function): (u8, u8) = unsafe { transmute((start_address + i*4) as u16) };
      v.push(self.read(bus, device, function, offset).unwrap());
    }
    v
  }

  fn read_as<T>(&mut self, bus: u8, device: u8, start_address: u16) -> Box<T> {
    let v = self.read_bytes(bus, device, start_address, size_of::<T>() as u16);
    let slice = &v[..];
    let read = read_into(slice);
    return read;
  }

  fn read_header(&mut self, bus: u8, device: u8) -> Option<PciHeader> {
    let (vendor, _): (u16, u16) = unsafe { transmute(self.read(bus, device, 0, 0).unwrap()) };
    
    
    if vendor == 0xffff {
      return None
    }

    let shared: SharedHeader = *self.read_as(bus, device, 0);
    //println!("{:?}", shared.header_type);
    let rest = match shared.header_type {
      0x00 => HeaderType::Basic(*self.read_as(bus, device, size_of::<SharedHeader>() as u16)),
      0x01 => HeaderType::Todo,
      0x02 => HeaderType::Todo,
      _ => {
        //println!("weird header");
        return None
      }
    };
    
    Some(PciHeader { shared: shared, rest: rest })
  }

}

impl DriverManager for Pci {

  fn get_drivers(&mut self) -> Vec<Box<NetworkDriver + 'static>> {
    let mut no_device_count: usize = 0;
    let mut device_count: usize = 0;

    let mut io_offset: u32 = 0;
    for bus in 0..255usize {
      for device in 0..32usize {
        
        //println!("{:?}...{:?}", bus,device);
        match self.read_header(bus as u8, device as u8) {
          None => no_device_count += 1,
          Some(header) => {
            
            device_count += 1;
            let shared = header.shared;
            // println!("bus #{} found device 0x{:x} -- vendor 0x{:x}", bus, shared.device, shared.vendor);
            // print!("    class 0x{:x}, subclass 0x{:x}", shared.class_code, shared.subclass);
            // print!("    header type 0x{:x}", shared.header_type);
            // print!("    status 0x{:x}, command 0x{:x}", shared.status, shared.command);
            
            match header.rest {
              HeaderType::Basic(next) => {
                
                if (shared.vendor == 0x10ec) && (shared.device == 0x8139 ) {
                  io_offset = (next.base_addresses[0] >> 2) << 2 ;
                  self.address_port.out32(Pci::build_address(bus as u8, device as u8, 0, 4)) ;
                  self.data_port.out16(shared.command | 0x4) ;
                  if io_offset != 0 { 


                    let manifest = Rtl8139::manifest();
                    let granter = PortGranter { base: io_offset as usize, limit: manifest.register_limit as usize };
                    
                    let mut x = NetworkStack::new(box Rtl8139::new(granter)) ; 
                    println!("Listening...");
                    //x.listen() ;
                    x.test() ; 
                    
                  }
                }

              }
              _ => ()
            }
          }
        }
      }
    }
     
    let mut ret: Vec<Box<NetworkDriver>> = Vec::new();
    // println!("io_offset: {:?}", io_offset);
    // if io_offset != 0 { 


    //   let manifest = Rtl8139::manifest();
    //   let granter = PortGranter { base: io_offset as usize, limit: manifest.register_limit as usize };
      
    //   let mut x = NetworkStack::new(box Rtl8139::new(granter)) ; 
    //   println!("Testing now...");
    //   x.test() ; 

      
    // }
   ret
  }

}
