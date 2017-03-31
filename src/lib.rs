#![feature(lang_items)]
#![no_std]
#![feature(const_fn)]
#![feature(unique)]
#![feature(alloc, collections)]
#![feature(abi_x86_interrupt)]

extern crate rlibc ; 
extern crate spin;
extern crate multiboot2;
extern crate x86_64 ;
#[macro_use] 
extern crate bitflags ; 


extern crate hole_list_allocator;
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
pub mod memory ; 
mod interrupts;

 
use memory::FrameAllocator ; 
use memory::area_frame_allocator::AreaFrameAllocator ;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    // ATTENTION: we have a very small stack and no guard page
    vga_buffer::clear_screen();
    println!("Hello World{}", "!");

    let boot_info = unsafe {
        multiboot2::load(multiboot_information_address)
    };
    enable_nxe_bit();
    enable_write_protect_bit();

    // set up guard page and map the heap pages
    let mut memory_controller = memory::init(boot_info);

    // initialize our IDT
    interrupts::init(&mut memory_controller);

    
    // fn stack_overflow() {
    //     stack_overflow(); // for each recursion, the return address is pushed
    // }

    // // trigger a stack overflow
    // stack_overflow();


    println!("It did not crash!");

    loop {}
}




#[lang="eh_personality"] pub extern fn eh_personality(){} 

#[lang = "panic_fmt"]
#[no_mangle]
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str,
    line: u32) -> !
{
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("    {}", fmt);
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