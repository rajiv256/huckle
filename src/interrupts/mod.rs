use x86_64::structures::idt::Idt;
use x86_64::structures::idt::ExceptionStackFrame;

use memory::MemoryController;
use x86_64::structures::tss::TaskStateSegment;

use x86_64::VirtualAddress;
const DOUBLE_FAULT_IST_INDEX: usize = 0 ;


use spin::Once ;
use rtl8139::Rtl8139;
use driver::NetworkDriver;


static TSS: Once<TaskStateSegment> = Once::new();
static GDT: Once<gdt::Gdt> = Once::new() ;

mod gdt ;

use peripherals::mycpu::Port ;
use pic8259::ChainedPics ;
static mut chainedPics : ChainedPics = unsafe { ChainedPics::new(0x20,0x28) } ;
use ps2_controller::* ;

lazy_static! {
    static ref IDT: Idt = {
        let mut idt = Idt::new();
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX as u16);
        }
        idt.breakpoint.set_handler_fn(breakpoint_handler);

idt.interrupts[0].set_handler_fn(timer_handler) ;
idt.interrupts[1].set_handler_fn(keyboard_handler) ;
idt.interrupts[2].set_handler_fn(handler2) ;
idt.interrupts[3].set_handler_fn(handler3) ;
idt.interrupts[4].set_handler_fn(handler4) ;
idt.interrupts[5].set_handler_fn(handler5) ;
idt.interrupts[6].set_handler_fn(handler6) ;
idt.interrupts[7].set_handler_fn(handler7) ;
idt.interrupts[8].set_handler_fn(handler8) ;
idt.interrupts[9].set_handler_fn(handler9) ;
idt.interrupts[10].set_handler_fn(handler10) ;
idt.interrupts[11].set_handler_fn(handler11) ;
idt.interrupts[12].set_handler_fn(handler12) ;
idt.interrupts[13].set_handler_fn(handler13) ;
idt.interrupts[14].set_handler_fn(handler14) ;
idt.interrupts[15].set_handler_fn(handler15) ;

        idt
    };
}
pub fn init(memory_controller: &mut MemoryController) {
    use x86_64::structures::gdt::SegmentSelector;
    use x86_64::instructions::segmentation::set_cs;
    use x86_64::instructions::tables::load_tss;

    let double_fault_stack = memory_controller.alloc_stack(1)
        .expect("could not allocate double fault stack");

    let tss = TSS.call_once(|| {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX] = VirtualAddress(
            double_fault_stack.top());
        tss
    });
    let mut code_selector = SegmentSelector(0);
    let mut tss_selector = SegmentSelector(0);
    let gdt = GDT.call_once(|| {
        let mut gdt = gdt::Gdt::new();
        code_selector = gdt.add_entry(gdt::Descriptor::
                            kernel_code_segment());
        tss_selector = gdt.add_entry(gdt::Descriptor::tss_segment(&tss));
        gdt
    });
    gdt.load();

     unsafe {
        // reload code segment register
        set_cs(code_selector);
        // load TSS
        load_tss(tss_selector);
    }

    IDT.load();


    unsafe { chainedPics.remap() } ;

}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame){
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn timer_handler(stack_frame: &mut ExceptionStackFrame){
    // println!("Timer Tik-tok Tik-tok!!!");
    unsafe { chainedPics.ChainedPics_sendEOI(0) ; }
}
extern "x86-interrupt" fn keyboard_handler(stack_frame: &mut ExceptionStackFrame){
    //println!("A key is pressed!!!");
    println!("The key is := {:?}\n", Port::new(0x60).in8()) ;

    unsafe { chainedPics.ChainedPics_sendEOI(1) ; }

}

extern "x86-interrupt" fn double_fault_handler(
    stack_sframe: &mut ExceptionStackFrame, _error_code: u64){
    println!("\nEXCEPTION: DOUBLE FAULT\n{:#?}", stack_sframe);
    loop {}
}

extern "x86-interrupt" fn handler2(stack_frame: &mut ExceptionStackFrame){ println!("In handler 2"); unsafe { chainedPics.ChainedPics_sendEOI(2) ; }}
extern "x86-interrupt" fn handler3(stack_frame: &mut ExceptionStackFrame){ println!("In handler 3"); unsafe { chainedPics.ChainedPics_sendEOI(3) ; }}
extern "x86-interrupt" fn handler4(stack_frame: &mut ExceptionStackFrame){ println!("In handler 4"); unsafe { chainedPics.ChainedPics_sendEOI(4) ; }}
extern "x86-interrupt" fn handler5(stack_frame: &mut ExceptionStackFrame){ println!("In handler 5"); unsafe { chainedPics.ChainedPics_sendEOI(5) ; }}
extern "x86-interrupt" fn handler6(stack_frame: &mut ExceptionStackFrame){ println!("In handler 6"); unsafe { chainedPics.ChainedPics_sendEOI(6) ; }}
extern "x86-interrupt" fn handler7(stack_frame: &mut ExceptionStackFrame){ println!("In handler 7"); unsafe { chainedPics.ChainedPics_sendEOI(7) ; }}
extern "x86-interrupt" fn handler8(stack_frame: &mut ExceptionStackFrame){ println!("In handler 8"); unsafe { chainedPics.ChainedPics_sendEOI(8) ; }}
extern "x86-interrupt" fn handler9(stack_frame: &mut ExceptionStackFrame){ println!("In handler 9"); unsafe { chainedPics.ChainedPics_sendEOI(9) ; }}
extern "x86-interrupt" fn handler10(stack_frame: &mut ExceptionStackFrame){ println!("In handler 10"); unsafe { chainedPics.ChainedPics_sendEOI(10) ; }}
extern "x86-interrupt" fn handler11(stack_frame: &mut ExceptionStackFrame){ println!("In handler 11"); unsafe { chainedPics.ChainedPics_sendEOI(11) ; }}
extern "x86-interrupt" fn handler12(stack_frame: &mut ExceptionStackFrame){ println!("In handler 12"); unsafe { chainedPics.ChainedPics_sendEOI(12) ; }}
extern "x86-interrupt" fn handler13(stack_frame: &mut ExceptionStackFrame){ println!("In handler 13"); unsafe { chainedPics.ChainedPics_sendEOI(13) ; }}
extern "x86-interrupt" fn handler14(stack_frame: &mut ExceptionStackFrame){ println!("In handler 14"); unsafe { chainedPics.ChainedPics_sendEOI(14) ; }}
extern "x86-interrupt" fn handler15(stack_frame: &mut ExceptionStackFrame){ println!("In handler 15"); unsafe { chainedPics.ChainedPics_sendEOI(15) ; }}
