### What is this project? 

- A unikernel web server written completely in Rust. 
- A standalone implementation that is can be directly deployed on bare-metal hardware. 
- One of the early instances of a fully functioning web server. 

### Components

1. A minimal kernel by following this excellent series of  [blog posts](https://os.phil-opp.com/) by Phill Oppermann
  - Basic Grub, bootloader, Long mode, Stack​
  - GDT, TSS
  - Paging for memory management with 4 GB of physical memory is mapped
  
2. VGA Display screen 
3. PCI to detect ethernet card and other peripheral devices. 
4. PIC 8259 Cascaded master and slave at IRQ 2 line.
5. Detection of PS/2 controller (interface for keyboard device and PIC) and initialization. 
6. Memory Mapped I/O
7. Network driver for RTL8139 Ethernet card. 
8. UDP web server capable of exchanging packets.

### Install Requirements

*Note: I used Ubuntu 16.04 as my development system, so adapt to your own system while installing packages.(I particulary struggled to make this work on a Mac.)*

Install `qemu`, `xorriso` and `nasm` 

`sudo apt install qemu xorisso nasm`

### Environment Setup

Install Rust from the official website. 

`curl https://sh.rustup.rs -sSf | sh`

> This will install rustup(the tool chain installer), rustc(the compiler), cargo(the package manager).
> Now we are using so many functions that are unstable, so we need to use the nightly versions of the compiler and the package manager. 
> Rustup helps in navigating between different versions of `rustc` and `cargo` depending on the requirement. 

Clone this repository and inside the root folder run the below command to override the version for this specific project. 

`rustup override add nightly-2017-04-05`

You can verify by checking the versions of `rustc` and `cargo`. However, there is a lot of activity over Rust in the recent past. *So if you are using latest versions, get ready to do some heavy debugging.*

```javascript
rajiv@rajiv-Inspiron-3537:~/huckle$ rustc --version`

rustc 1.18.0-nightly (2564711e8 2017-04-04)

rajiv@rajiv-Inspiron-3537:~/huckle$ cargo --version`

cargo 0.19.0-nightly (4e95c6b41 2017-03-23)
```

If these versions match, you are good to go!

### How to run? 

Go to the root directory and run 

`make iso` 

`make run` 

Once you are done, clean up the binaries. 

`make clean`


### Notes

- Used Qemu as the hardware emulator as it gives the convenience of specifying the desired hardware

### Credits 

- [Writing an OS in Rust](https://os.phil-opp.com/), by Phillip Oppermann. 

- The following blogs helped me in organizing my project and also helped by pointing me in the right direction when I was    stuck in a quagmire of triple faults. 
  - https://jvns.ca/blog/2014/03/12/the-rust-os-story/
  - https://github.com/ryanra/RustOS (Helped me a lot in organizing my code and understanding how PCI devices work)


Long live Open Source!

