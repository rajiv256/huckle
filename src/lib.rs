#![feature(lang_items)]
#![no_std]
#![feature(const_fn)]
#![feature(unique)]
extern crate rlibc ; 
extern crate spin;
extern crate multiboot2;

#[macro_use]
mod vga_buffer ; 


#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {
	// use ::vga_buffer::print_something ; 
 //    print_something() ; 

    vga_buffer::clear_screen();

    let boot_info = unsafe{ multiboot2::load(multiboot_information_address) };
    let memory_map_tag = boot_info.memory_map_tag()
    .expect("Memory map tag required");

    let elf_sections_tag = boot_info.elf_sections_tag()
    .expect("Elf-sections tag required");

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr)
        .min().unwrap();
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size)
        .max().unwrap();

    let multiboot_start = multiboot_information_address;
    let multiboot_end = multiboot_start + (boot_info.total_size as usize);

    println!("kernel_start: {}, kernel_end: {}", kernel_start, kernel_end);
    println!("multiboot_start: {}, multiboot_end: {}", multiboot_start, multiboot_end);
       
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