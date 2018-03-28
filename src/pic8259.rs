use peripherals::mycpu::Port ;

// From https://github.com/emk/toyos-rs/blob/master/crates/pic8259_simple/src/lib.rs
const CMD_INIT : u8 = 0x11 ;   // Initialize command for PIC
const CMD_END_OF_INTERRUPT : u8 = 0x20 ;    // Command for End of Interrupt
const MODE_8086 : u8 = 0x01 ;     // The mode of operation

const PIC1 : u8 = 0x20 ;
const PIC2 : u8 = 0xA0 ;
const PIC1_CMD : u16 = 0x0020 ;
const PIC1_DATA : u16 = 0x0021 ;
const PIC2_CMD : u16 = 0x00A0 ;
const PIC2_DATA : u16 = 0x00A1 ;
const PIC1_OFFSET : u8 = 0x20 ;
const PIC2_OFFSET : u8 = 0x28 ;

// Structure of an individual PIC
struct Pic {

    offset : u8,            // The offset to which the interrupts are mapped.
    command_port : Port,    // IO port to which we send commands
    data_port : Port,       // IO port from which we send and receive data
}

impl Pic {
    // Am I incharge of handling an interrupt
    fn handles_interrupt(&self , interrupt_id: u8) -> bool {
        self.offset <=interrupt_id && interrupt_id < self.offset+8
    }

    // Notify us that the interrupt is handled
    unsafe fn PIC_sendEOI(&mut self){
        // println!("Inside the PIC");
        self.command_port.out8(CMD_END_OF_INTERRUPT) ;
        Port::io_wait() ;
    }
}

// A pair of chained Pics...slave's output connected to third line or IRQ #2
pub struct ChainedPics {
    pics : [Pic; 2] ,
}

impl ChainedPics {
    // Create new interface for the standard PIC1 and PIC2 controllers
    // specifying the desired interrupt offsets
    pub const unsafe fn new(offset1: u8 , offset2: u8) -> ChainedPics {
        ChainedPics {
            pics : [
                Pic{
                    offset : offset1 ,
                    command_port: Port::new(PIC1_CMD),
                    data_port : Port::new(PIC1_DATA),
                },
                Pic {
                    offset : offset2,
                    command_port: Port::new(PIC2_CMD),
                    data_port : Port::new(PIC2_DATA),
                },
            ],
        }
    }

    // Remapping the PIC with offsets.
    pub unsafe fn remap(&mut self){

        let saved_mask1 = self.pics[0].data_port.in8() ;
        let saved_mask2 = self.pics[1].data_port.in8() ;   // Refer Barebones Rust and OS Wiki Dev

        println!("mask1 := {:0x}", saved_mask1);
        println!("mask2 := {:0x}", saved_mask2);
        // Tell each PIC that we are going to send a 3-byte initialization sequence
        //  on each data port.
        self.pics[0].command_port.out8(CMD_INIT) ;
        Port::io_wait() ;
        self.pics[1].command_port.out8(CMD_INIT) ;
        Port::io_wait() ;

        // Byte 1 : Set up our base offsets
        self.pics[0].data_port.out8(self.pics[0].offset) ;
        Port::io_wait() ;
        self.pics[1].data_port.out8(self.pics[1].offset) ;
        Port::io_wait() ;

        // Byte 2 : Configure chaining between the PIC1 and PIC2
        self.pics[0].data_port.out8(4) ;
        Port::io_wait() ;
        self.pics[1].data_port.out8(2) ;
        Port::io_wait() ;

        // Byte 3 : Set our mode
        self.pics[0].data_port.out8(MODE_8086) ;
        Port::io_wait() ;
        self.pics[1].data_port.out8(MODE_8086) ;
        Port::io_wait() ;

        // Restore our saved masks
        self.pics[0].data_port.out8(0x00) ;
        Port::io_wait() ;
        self.pics[1].data_port.out8(0x00) ;
        Port::io_wait() ;

    }

    pub fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.pics.iter().any(|p| p.handles_interrupt(interrupt_id))
    }

    pub unsafe fn ChainedPics_sendEOI(&mut self, interrupt_id : u8){
        //println!("Called once ** {:?}", interrupt_id);
        if self.handles_interrupt(interrupt_id+0x20) {
            if self.pics[1].handles_interrupt(interrupt_id+0x20){
                self.pics[1].PIC_sendEOI() ;
            }
            self.pics[0].PIC_sendEOI() ;
            Port::io_wait() ;
            Port::io_wait() ;
        }
    }

}
