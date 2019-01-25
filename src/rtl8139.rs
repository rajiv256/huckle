#![feature(inclusive_range_syntax)]

use peripherals::mycpu::Port;
use driver::{Driver, NetworkDriver};
use pci::{PciManifest, PortGranter};

///////////////////////////MACROS////////////////////////////////////

/* 8139 register offsets */
const TSD0 :u16   = 0x10 ;
const TSAD0 :u16  = 0x20 ;
const RBSTART:u16 = 0x30 ;
const CR :u16     = 0x37 ;
const CAPR:u16    = 0x38 ;
const IMR:u16     = 0x3C ;
const ISR:u16     = 0x3E ;
const TCR:u16     = 0x40 ;
const RCR:u16     = 0x44 ;
const MPC:u16     = 0x4C ;
const MULINT:u16  = 0x5C ;
const CBR: u16    = 0x3A ;

 /* TSD register commands */
const TxHostOwns:u32  = 0x2000 ;
const TxUnderrun:u32  = 0x4000 ;
const TxStatOK:u32    = 0x8000 ;
const TxOutOfWindow:u32 =  0x20000000 ;
const TxAborted:u64   = 0x40000000 ;
const TxCarrierLost:u64 = 0x80000000 ;

  /* CR register commands */
const RxBufEmpty:u16 =  0x01 ;
const CmdTxEnb:u16 = 0x04 ;
const CmdRxEnb:u16 = 0x08 ;
const CmdReset:u16 = 0x10 ;

/* ISR Bits */
const RxOK :u16    = 0x01 ;
const RxErr:u16    = 0x02 ;
const TxOK :u16    = 0x04 ;
const TxErr:u16    = 0x08 ;
const RxOverFlow:u16 = 0x10 ;
const RxUnderrun:u16 = 0x20 ;
const RxFIFOOver:u16 = 0x40 ;
const CableLen:u32 = 0x2000 ;
const TimeOut:u32  = 0x4000 ;
const SysErr:u32   = 0x8000 ;

const RX_BUF_LEN_IDX:usize = 2 ;          /* 0==8K, 1==16K, 2==32K, 3==64K */
const RX_BUF_LEN:usize  =   (1024 << RX_BUF_LEN_IDX) ;
const RX_BUF_PAD:usize  =   16 ;           /* see 11th and 12th bit of RCR: 0x44 */
const RX_BUF_WRAP_PAD:usize =  256 ;    /* spare padding to handle pkt wrap */
const RX_BUF_TOT_LEN:usize =  (RX_BUF_LEN + RX_BUF_PAD + RX_BUF_WRAP_PAD) ;



const INT_MASK:u32 = (RxOK as u32 | RxErr as u32 | TxOK as u32 | TxErr as u32 | RxOverFlow as u32 | RxUnderrun as u32 | RxFIFOOver as u32 | CableLen | TimeOut | SysErr) ;

///////////////////END_OF_MACROS////////////////////////////////////////////



pub struct Rtl8139 {
  command_register: Port, // TODO(ryan): better abstraction for registers (i.e., should take byte-width into consideration + also be for mmap)
  transmit_address: [Port; 4],
  transmit_status: [Port; 4],
  id: [Port; 6],
  config_1: Port,
  descriptor: usize,
  config_rx: Port,
  rx_ring: [u8;RX_BUF_TOT_LEN],
  cur_rx: usize,
  rbstart: Port,
  imr: Port,
  mpc:Port,
  mulint:Port,
  isr: Port,
  ersr: Port,
  capr: Port,
  cbr : Port,
}

impl Rtl8139 { // TODO(ryan): is there already a frame oriented interface in std libs to implement?

  pub fn manifest() -> PciManifest {
    PciManifest { register_limit: 0x100, device_id: 0x8139, vendor_id: 0x10ec, bus_master: true }
  }


  pub fn new(granter: PortGranter) -> Rtl8139 {

    let p = |off: u16| -> Port {
      granter.get(off as usize)
    };

    let mut card = Rtl8139 {
      config_1: p(0x52),
      command_register: p(0x37),
      transmit_address: [p(0x20), p(0x24), p(0x28), p(0x2c)],
      transmit_status:  [p(0x10), p(0x14), p(0x18), p(0x1c)],
      id: [p(0), p(1), p(2), p(3), p(4), p(5)],
      descriptor: 0,
      rx_ring:[0;RX_BUF_TOT_LEN],
      config_rx: p(RCR),
      cur_rx: 0,
      rbstart:p(RBSTART),
      mulint: p(MULINT),
      imr: p(IMR),
      mpc: p(MPC),
      isr: p(ISR),
      ersr:p(0x0036),
      capr: p(CAPR),
      cbr : p(CBR)

    };
    card.init() ;

    //card.listen();   // This listen() function is enabled in the huckle-listen repo as it is the server and this repo is the client. 
    card
  }




}

impl Driver for Rtl8139 {

  fn init(&mut self) {

    self.config_1.out8(0x00);    // This should power-on the device. Basically LWAKE+LWPTN pin are set active.
    Port::io_wait() ;


    self.command_register.out8(0x10); // Software reset
    Port::io_wait() ;



    while (self.command_register.in8() & 0x10) != 0 { } // Wait till the RST(RESET) bit is set to 0.

    //configuring RBSTART. Put the start address of recv buffer into the RBSTART port.
    self.rbstart.out32(self.rx_ring.as_ptr() as u32) ;
    Port::io_wait() ;


    self.command_register.out8(0x0C); // enable transmit and receive. --> 0x08|0x04 (Important)
    Port::io_wait() ;

    while (self.command_register.in8() & 0x0c) != 0x0c {}


    //Enable all possible interrupts by setting the interrupt mask.
    self.imr.out16(0x0005) ;  // Need to set it as TxOK | RxOK = 0x0005
    Port::io_wait() ;

    //This is the RCR register. Setting the values of AB(Accept Broadcast message, AM (Acc Multicast message),
    // APM (Acc packet which matches with MAC) , AAP (Acc all packets), Wrap (1<<7))
    // self.config_rx.out32((1 << 12) | (1 << 8) | (1 << 7) | (1 << 3) | (1 << 2) | (1 << 0)|(1<<1)| 0x4000 ) ;
    // Refer to the Datasheet
    self.config_rx.out32(0x0002b6b1) ;
    Port::io_wait() ;



    //init missed packet counter
    self.mpc.out16(0x00) ;
    Port::io_wait() ;

    // No early rx-interrupts
    // self.mulint.out16(self.mulint.in16()&0xf000) ;
    // Port::io_wait() ;

  }
  fn listen(&mut self) {
    println!("The ersbr(tikka) is at :- 0x{:x}",self.ersr.in16()) ;
    loop{}
  }


}

impl NetworkDriver for Rtl8139
{
  fn put_frame(&mut self, buf: &[u8]) -> Result<usize, u32> {

    for i in 0..buf.len() {
        print!("{:?}|",buf[i] as u8) ;
    }
    self.transmit_address[self.descriptor].out32(buf.as_ptr() as u32); // Give the address of the beginning of the packet.
    Port::io_wait() ;

    // while(self.transmit_status[self.descriptor].in32() & (1<<15) != 0x0) {}
    println!("{:?}*******transmit_status before 0x{:x}",self.descriptor as u32, self.transmit_status[self.descriptor].in32() as u32) ;


    self.transmit_status[self.descriptor].out32(0x40000 | (buf.len() as u32));  // Size of the packet
    Port::io_wait() ;


    // while (self.transmit_status[self.descriptor].in32() != 0xa02e){}
    println!("transmit_status 0x{:x}",self.transmit_status[self.descriptor].in32() as u32) ;

    self.transmit_status[self.descriptor].out32(self.transmit_status[self.descriptor].in32()^(1<<13));
    Port::io_wait() ;
    println!("transmit_status after 0x{:x}",self.transmit_status[self.descriptor].in32() as u32 & 0x2e) ;

    println!("Transmitted!"  );


    self.descriptor = (self.descriptor + 1) % 4 ;
    if self.descriptor == 0 {
        self.reset_init() ;
    }
    Ok(buf.len())
  }
  fn reset_init(&mut self){
      self.command_register.out8(0x10); // Software reset
      Port::io_wait() ;

      while (self.command_register.in8() & 0x10) != 0 { } // Wait till the RST(RESET) bit is set to 0.

      //configuring RBSTART. Put the start address of recv buffer into the RBSTART port.
      self.rbstart.out32(self.rx_ring.as_ptr() as u32) ;
      Port::io_wait() ;


      self.command_register.out8(0x0C); // enable transmit and receive. --> 0x08|0x04 (Important)
      Port::io_wait() ;

      while (self.command_register.in8() & 0x0c) != 0x0c {}


      //Enable all possible interrupts by setting the interrupt mask.
      self.imr.out16(0x0005) ;  // Need to set it as TxOK | RxOK = 0x0005
      Port::io_wait() ;

      //This is the RCR register. Setting the values of AB(Accept Broadcast message, AM (Acc Multicast message),
      // APM (Acc packet which matches with MAC) , AAP (Acc all packets), Wrap (1<<7))
      // self.config_rx.out32((1 << 12) | (1 << 8) | (1 << 7) | (1 << 3) | (1 << 2) | (1 << 0)|(1<<1)| 0x4000 ) ;
      self.config_rx.out32(0x0002b6b1) ;
      Port::io_wait() ;

      //init missed packet counter
      self.mpc.out16(0x00) ;
      Port::io_wait() ;

      // No early rx-interrupts
      // self.mulint.out16(self.mulint.in16()&0xf000) ;
      // Port::io_wait() ;

  }
  fn nic_interrupt_handler(&mut self) {


      let mut isr: u32 = self.isr.in32() ;

     //Some thing to be done for transmission interrupt.

     //For receive.
     if (isr & RxErr as u32 !=0) {
        /* TODO: Need detailed analysis of error status */
        println!("receive err interrupt");
     }
     if (isr & RxOK as u32 != 0) {
        println!("Interrupt handler entered!");
        while ((self.command_register.in16() & RxBufEmpty)==0) {
          let mut rx_status : u32 = 0 ;
          let mut rx_size: u16 = 0 ;
          let mut pkt_size: u16 = 0 ;
          if (self.cur_rx > RX_BUF_LEN) {
            self.cur_rx = self.cur_rx%RX_BUF_LEN  ;
          }

           /* TODO: need to convert rx_status from little to host endian
            * XXX: My CPU is little endian only :-)
            */
          // The below line may or may not be correct.
          rx_status = unsafe {*((self.rx_ring.as_ptr() as u32+self.cur_rx as u32) as *mut u32)};
          rx_size = ((rx_status & 0xff00)/65536) as u16;

          pkt_size = rx_size - 4;

          //TODO : Handover packet to the system.


        }
     }


    /* clear all interrupt.
     * Specs says reading ISR clears all interrupts and writing
     * has no effect. But this does not seem to be case. I keep on
     * getting interrupt unless I forcibly clears all interrupt :-(
     */
     self.isr.out16(0xffff) ;

  }
  fn address(&mut self) -> [u8; 6] {
    let mut ret = [0; 6];
    for i in 0..6usize {
      ret[i] = self.id[i].in8() as u8;
    }
     println!("{:?}", ret);
    ret
  }
}
