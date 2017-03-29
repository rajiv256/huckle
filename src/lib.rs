#![feature(lang_items)]
#![no_std]
#![feature(const_fn)]
#![feature(unique)]
extern crate rlibc ; 
extern crate spin;
extern crate multiboot2;
extern crate x86_64 ;
#[macro_use] 
extern crate bitflags ; 

extern crate x86 ; 
#[macro_use]
pub mod vga_buffer ; 
pub mod memory ; 
 
use memory::FrameAllocator ; 
use memory::area_frame_allocator::AreaFrameAllocator ;

#[no_mangle]
pub extern "C" fn rust_main(multiboot_information_address: usize) {
    // ATTENTION: we have a very small stack and no guard page

    // the same as before
    vga_buffer::clear_screen();
    println!("Hello World{}", "!");

    let boot_info = unsafe {
        multiboot2::load(multiboot_information_address)
    };
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");
    let elf_sections_tag = boot_info.elf_sections_tag()
        .expect("Elf sections tag required");

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
        .min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
        .max().unwrap();

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    println!("kernel start: 0x{:x}, kernel end: 0x{:x}",
        kernel_start, kernel_end);
    println!("multiboot start: 0x{:x}, multiboot end: 0x{:x}",
        multiboot_start, multiboot_end);

    let mut frame_allocator = memory::area_frame_allocator::AreaFrameAllocator::new(
        kernel_start as usize, kernel_end as usize, multiboot_start,
        multiboot_end, memory_map_tag.memory_areas());

    // this is the new part
    enable_nxe_bit();
    enable_write_protect_bit();
    memory::remap_the_kernel(&mut frame_allocator, boot_info);
    frame_allocator.allocate_frame(); 
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