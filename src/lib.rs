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
use peripherals::mycpu::Port ; 


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
    
     
    let mut p: Pci =  Pci::new() ; 
    p.get_drivers() ; 
    
    println!("It didn't crash");
    loop {}
    
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