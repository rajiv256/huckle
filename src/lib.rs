
#![feature(lang_items)]
#![no_std]
#![feature(const_fn)]
#![feature(unique)]
#![feature(alloc, collections)]
#![feature(abi_x86_interrupt)]
#![feature(box_syntax)]
#![feature(asm)]

extern crate rlibc ;
extern crate spin;
extern crate multiboot2;
#[macro_use]
extern crate x86_64 ;
#[macro_use]
extern crate bitflags ;


extern crate bump_allocator;
extern crate alloc;
#[macro_use]
extern crate collections;
#[macro_use]
extern crate once;

#[macro_use]
extern crate lazy_static;

extern crate x86 ;
#[macro_use]
pub mod vga_buffer ;


extern crate bit_field ;

extern crate cpu ;
extern crate coreio ;
extern crate cpuio ;


pub mod memory ;
mod interrupts;

pub mod peripherals ;
mod io ;
mod driver ;
mod panic ;
mod pci ;
mod rtl8139 ;
mod terminal ;
pub mod net ;

use pci::* ;
use driver::DriverManager;

use memory::FrameAllocator ;
use memory::area_frame_allocator::AreaFrameAllocator ;

use ::net::NetworkStack ;
use spin::Mutex;

pub mod pic8259 ;
use pic8259::* ;

use peripherals::mycpu::Port;

mod ps2_controller ;


#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    // ATTENTION: we have a very small stack and no guard page
    vga_buffer::clear_screen();
    //println!("Hello World{}", "!");

    let boot_info = unsafe {
        multiboot2::load(multiboot_information_address)
    };
    enable_nxe_bit();
    enable_write_protect_bit();

    // set up guard page and map the heap pages
    let mut memory_controller = memory::init(boot_info);

    // initialize our IDT
    interrupts::init(&mut memory_controller);

    unsafe { ps2_controller::ps2_reset() ; }


    unsafe { asm!("sti" :::: "volatile", "intel"); }

    let mut pci: Pci =  Pci::new() ;
    println!("Getting drivers -- lib.rs");
    pci.get_drivers() ;




    // loop {}

    // We probably have to call other processes here.

     println!("It didn't crash");
}




#[lang="eh_personality"] pub extern fn eh_personality(){}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str,
line: u32) -> !
{
    //println!("\n\nPANIC in {} at line {}:", file, line);
    //println!("    {}", fmt);
    loop{}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}


fn enable_nxe_bit() {
    use x86_64::registers::msr::{IA32_EFER, rdmsr, wrmsr};

    let nxe_bit = 1 << 11;
    unsafe {
        let efer = rdmsr(IA32_EFER);
        wrmsr(IA32_EFER, efer | nxe_bit);
    }
}

fn enable_write_protect_bit() {
    use x86_64::registers::control_regs::{cr0, cr0_write, Cr0};

    unsafe { cr0_write(cr0() | Cr0::WRITE_PROTECT) };
}





/////////////////////////This is the basic Hello world that works//////////////////////////////////////////////
// // ATTENTION: we have a very small stack and no guard page

//    let hello = b"Hello World!";
//    let color_byte = 0x1f; // white foreground, blue background

//    let mut hello_colored = [color_byte; 24];
//    for (i, char_byte) in hello.into_iter().enumerate() {
//        hello_colored[i*2] = *char_byte;
//    }

//    // write `Hello World!` to the center of the VGA text buffer
//    let buffer_ptr = (0xb8000 + 1988) as *mut _;
//    unsafe { *buffer_ptr = hello_colored };

//    loop{}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////
///////////////////////// This is the PS2_controller code /////////////////////////////////////
/*
let ps2_data : Port = Port::new(0x0060) ;
let ps2_cmd  : Port = Port::new(0x0064) ;

// Disable the devices.
ps2_cmd.out8(0xAD) ;
Port::io_wait() ;

ps2_cmd.out8(0xA7) ;
Port::io_wait() ;

// Flush the output buffer
ps2_data.in8() ;
Port::io_wait() ;

// Changing the Configuration byte so that the secod PS/2 port 0, 1 , 6 bits should be disabled.
ps2_cmd.out8(0x60) ;
Port::io_wait() ;

ps2_data.out8(0x20) ;
Port::io_wait() ;

ps2_cmd.out8(0x20) ;
Port::io_wait();
println!("ps2_byte0 := 0x{:0x}", ps2_data.in8());

// Perform Controller self test
ps2_cmd.out8(0xAA) ;
Port::io_wait() ;

let self_test : u8 = ps2_data.in8() ;
if self_test == 0x55 {
    println!("PS/2 Controller self test successful!");
}

// Checking if the coonfig byte is set to start-up defaults including enabling translation.
// If it is not reset then the value in the config byte should be 0x20 since it is what we have set before.
ps2_cmd.out8(0x20) ;
Port::io_wait() ;

println!("byte0 after self test := 0x{:0x}. It should be 0x20.", ps2_data.in8());

// Checking if we have two PS/2 channels
ps2_cmd.out8(0xA8) ;
Port::io_wait() ;

ps2_cmd.out8(0x20) ;
Port::io_wait() ;

println!("Bit 5 in config byte is clear. :=0x{:0x}. Should be 0", ps2_data.in8()&(1<<5));

// Perform Interface tests
ps2_cmd.out8(0xAB) ;
Port::io_wait() ;

let response : u8 = ps2_data.in8() ;
if response == 0x00 {
    println!("Testing second PS/2 port...Successfull. Expected:0x00  Found: 0x{:0x}\n", response);
}
ps2_cmd.out8(0xA9) ;
Port::io_wait() ;

let response : u8 = ps2_data.in8() ;
if response == 0x00 {
    println!("Testing second PS/2 port...Successfull. Expected:0x00  Found: 0x{:0x}\n", response);
}

// Enable device.
// 1. Enabling ports
ps2_cmd.out8(0xAE) ;
Port::io_wait() ;
ps2_cmd.out8(0xA8) ;
Port::io_wait() ;

// 2. Enabling interrupts by setting bits 0 and 1
ps2_cmd.out8(0x20) ;
Port::io_wait() ;
let mut config_byte = ps2_data.in8() ;
config_byte = config_byte | 0x03 ;
ps2_cmd.out8(0x60) ;
Port::io_wait() ;

ps2_data.out8(config_byte) ;
Port::io_wait() ;
ps2_cmd.out8(0x20) ;
Port::io_wait() ;

println!("Final config_byte := 0x{:0x}. Should be 3.\n", ps2_data.in8() );

// Reset the devices (This is device not the Controller)
// TODO : I don't know how exactly. But its ok. As long as we are confident that the devices are not corrupted.

// Identifying the devices
println!("status reg := {:?}", ps2_cmd.in8() );
ps2_data.out8(0xF5) ;
Port::io_wait() ;
while ps2_cmd.in8()&1==0 {
    Port::io_wait() ;
}
println!("Got Ack for disable scanning!! 0x{:0x}", ps2_data.in8());

ps2_data.out8(0xF2) ;
Port::io_wait() ;
while ps2_cmd.in8()&1==0 {
    Port::io_wait() ;
}
println!("Got Ack for device!! 0x{:0x}", ps2_data.in8());

while ps2_cmd.in8()&1==0 {
    Port::io_wait() ;
}
println!("Device first byte:= 0x{:0x}",ps2_data.in8()) ;

while ps2_cmd.in8()&1==0 {
    Port::io_wait() ;
}
println!("Device second byte:= 0x{:0x}",ps2_data.in8()) ;
println!("The PS/2 device found is MF2 Keyboard --> OS Dev Wiki", );
*/
//////////////////////////////////////////////////////////////////////////////////////
