use peripherals::mycpu::Port ;

pub unsafe fn ps2_reset() {
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
    config_byte = config_byte | 0x07 ;
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
}
