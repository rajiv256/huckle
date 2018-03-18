use x86_64::structures::idt::Idt;
use x86_64::structures::idt::ExceptionStackFrame;

use memory::MemoryController;
use x86_64::structures::tss::TaskStateSegment;

use x86_64::VirtualAddress;
const DOUBLE_FAULT_IST_INDEX: usize = 0 ;


use spin::Once ;


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
// idt.interrupts[2].set_handler_fn(keyboard_handler) ;
// idt.interrupts[3].set_handler_fn(keyboard_handler) ;
// idt.interrupts[4].set_handler_fn(keyboard_handler) ;
// idt.interrupts[5].set_handler_fn(keyboard_handler) ;
// idt.interrupts[6].set_handler_fn(keyboard_handler) ;
// idt.interrupts[7].set_handler_fn(keyboard_handler) ;
// idt.interrupts[8].set_handler_fn(keyboard_handler) ;
// idt.interrupts[9].set_handler_fn(keyboard_handler) ;
// idt.interrupts[10].set_handler_fn(keyboard_handler) ;
// idt.interrupts[11].set_handler_fn(keyboard_handler) ;
// idt.interrupts[12].set_handler_fn(keyboard_handler) ;
// idt.interrupts[13].set_handler_fn(keyboard_handler) ;
// idt.interrupts[14].set_handler_fn(keyboard_handler) ;
// idt.interrupts[15].set_handler_fn(keyboard_handler) ;
// idt.interrupts[16].set_handler_fn(keyboard_handler) ;
// idt.interrupts[17].set_handler_fn(keyboard_handler) ;
// idt.interrupts[18].set_handler_fn(keyboard_handler) ;
// idt.interrupts[19].set_handler_fn(keyboard_handler) ;
// idt.interrupts[20].set_handler_fn(keyboard_handler) ;
// idt.interrupts[21].set_handler_fn(keyboard_handler) ;
// idt.interrupts[22].set_handler_fn(keyboard_handler) ;
// idt.interrupts[23].set_handler_fn(keyboard_handler) ;
// idt.interrupts[24].set_handler_fn(keyboard_handler) ;
// idt.interrupts[25].set_handler_fn(keyboard_handler) ;
// idt.interrupts[26].set_handler_fn(keyboard_handler) ;
// idt.interrupts[27].set_handler_fn(keyboard_handler) ;
// idt.interrupts[28].set_handler_fn(keyboard_handler) ;
// idt.interrupts[29].set_handler_fn(keyboard_handler) ;
// idt.interrupts[30].set_handler_fn(keyboard_handler) ;
// idt.interrupts[31].set_handler_fn(keyboard_handler) ;
// idt.interrupts[32].set_handler_fn(keyboard_handler) ;
// idt.interrupts[33].set_handler_fn(keyboard_handler) ;
// idt.interrupts[34].set_handler_fn(keyboard_handler) ;
// idt.interrupts[35].set_handler_fn(keyboard_handler) ;
// idt.interrupts[36].set_handler_fn(keyboard_handler) ;
// idt.interrupts[37].set_handler_fn(keyboard_handler) ;
// idt.interrupts[38].set_handler_fn(keyboard_handler) ;
// idt.interrupts[39].set_handler_fn(keyboard_handler) ;
// idt.interrupts[40].set_handler_fn(keyboard_handler) ;
// idt.interrupts[41].set_handler_fn(keyboard_handler) ;
// idt.interrupts[42].set_handler_fn(keyboard_handler) ;
// idt.interrupts[43].set_handler_fn(keyboard_handler) ;
// idt.interrupts[44].set_handler_fn(keyboard_handler) ;
// idt.interrupts[45].set_handler_fn(keyboard_handler) ;
// idt.interrupts[46].set_handler_fn(keyboard_handler) ;
// idt.interrupts[47].set_handler_fn(keyboard_handler) ;
// idt.interrupts[48].set_handler_fn(keyboard_handler) ;
// idt.interrupts[49].set_handler_fn(keyboard_handler) ;
// idt.interrupts[50].set_handler_fn(keyboard_handler) ;
// idt.interrupts[51].set_handler_fn(keyboard_handler) ;
// idt.interrupts[52].set_handler_fn(keyboard_handler) ;
// idt.interrupts[53].set_handler_fn(keyboard_handler) ;
// idt.interrupts[54].set_handler_fn(keyboard_handler) ;
// idt.interrupts[55].set_handler_fn(keyboard_handler) ;
// idt.interrupts[56].set_handler_fn(keyboard_handler) ;
// idt.interrupts[57].set_handler_fn(keyboard_handler) ;
// idt.interrupts[58].set_handler_fn(keyboard_handler) ;
// idt.interrupts[59].set_handler_fn(keyboard_handler) ;
// idt.interrupts[60].set_handler_fn(keyboard_handler) ;
// idt.interrupts[61].set_handler_fn(keyboard_handler) ;
// idt.interrupts[62].set_handler_fn(keyboard_handler) ;
// idt.interrupts[63].set_handler_fn(keyboard_handler) ;
// idt.interrupts[64].set_handler_fn(keyboard_handler) ;
// idt.interrupts[65].set_handler_fn(keyboard_handler) ;
// idt.interrupts[66].set_handler_fn(keyboard_handler) ;
// idt.interrupts[67].set_handler_fn(keyboard_handler) ;
// idt.interrupts[68].set_handler_fn(keyboard_handler) ;
// idt.interrupts[69].set_handler_fn(keyboard_handler) ;
// idt.interrupts[70].set_handler_fn(keyboard_handler) ;
// idt.interrupts[71].set_handler_fn(keyboard_handler) ;
// idt.interrupts[72].set_handler_fn(keyboard_handler) ;
// idt.interrupts[73].set_handler_fn(keyboard_handler) ;
// idt.interrupts[74].set_handler_fn(keyboard_handler) ;
// idt.interrupts[75].set_handler_fn(keyboard_handler) ;
// idt.interrupts[76].set_handler_fn(keyboard_handler) ;
// idt.interrupts[77].set_handler_fn(keyboard_handler) ;
// idt.interrupts[78].set_handler_fn(keyboard_handler) ;
// idt.interrupts[79].set_handler_fn(keyboard_handler) ;
// idt.interrupts[80].set_handler_fn(keyboard_handler) ;
// idt.interrupts[81].set_handler_fn(keyboard_handler) ;
// idt.interrupts[82].set_handler_fn(keyboard_handler) ;
// idt.interrupts[83].set_handler_fn(keyboard_handler) ;
// idt.interrupts[84].set_handler_fn(keyboard_handler) ;
// idt.interrupts[85].set_handler_fn(keyboard_handler) ;
// idt.interrupts[86].set_handler_fn(keyboard_handler) ;
// idt.interrupts[87].set_handler_fn(keyboard_handler) ;
// idt.interrupts[88].set_handler_fn(keyboard_handler) ;
// idt.interrupts[89].set_handler_fn(keyboard_handler) ;
// idt.interrupts[90].set_handler_fn(keyboard_handler) ;
// idt.interrupts[91].set_handler_fn(keyboard_handler) ;
// idt.interrupts[92].set_handler_fn(keyboard_handler) ;
// idt.interrupts[93].set_handler_fn(keyboard_handler) ;
// idt.interrupts[94].set_handler_fn(keyboard_handler) ;
// idt.interrupts[95].set_handler_fn(keyboard_handler) ;
// idt.interrupts[96].set_handler_fn(keyboard_handler) ;
// idt.interrupts[97].set_handler_fn(keyboard_handler) ;
// idt.interrupts[98].set_handler_fn(keyboard_handler) ;
// idt.interrupts[99].set_handler_fn(keyboard_handler) ;
// idt.interrupts[100].set_handler_fn(keyboard_handler) ;
// idt.interrupts[101].set_handler_fn(keyboard_handler) ;
// idt.interrupts[102].set_handler_fn(keyboard_handler) ;
// idt.interrupts[103].set_handler_fn(keyboard_handler) ;
// idt.interrupts[104].set_handler_fn(keyboard_handler) ;
// idt.interrupts[105].set_handler_fn(keyboard_handler) ;
// idt.interrupts[106].set_handler_fn(keyboard_handler) ;
// idt.interrupts[107].set_handler_fn(keyboard_handler) ;
// idt.interrupts[108].set_handler_fn(keyboard_handler) ;
// idt.interrupts[109].set_handler_fn(keyboard_handler) ;
// idt.interrupts[110].set_handler_fn(keyboard_handler) ;
// idt.interrupts[111].set_handler_fn(keyboard_handler) ;
// idt.interrupts[112].set_handler_fn(keyboard_handler) ;
// idt.interrupts[113].set_handler_fn(keyboard_handler) ;
// idt.interrupts[114].set_handler_fn(keyboard_handler) ;
// idt.interrupts[115].set_handler_fn(keyboard_handler) ;
// idt.interrupts[116].set_handler_fn(keyboard_handler) ;
// idt.interrupts[117].set_handler_fn(keyboard_handler) ;
// idt.interrupts[118].set_handler_fn(keyboard_handler) ;
// idt.interrupts[119].set_handler_fn(keyboard_handler) ;
// idt.interrupts[120].set_handler_fn(keyboard_handler) ;
// idt.interrupts[121].set_handler_fn(keyboard_handler) ;
// idt.interrupts[122].set_handler_fn(keyboard_handler) ;
// idt.interrupts[123].set_handler_fn(keyboard_handler) ;
// idt.interrupts[124].set_handler_fn(keyboard_handler) ;
// idt.interrupts[125].set_handler_fn(keyboard_handler) ;
// idt.interrupts[126].set_handler_fn(keyboard_handler) ;
// idt.interrupts[127].set_handler_fn(keyboard_handler) ;
// idt.interrupts[128].set_handler_fn(keyboard_handler) ;
// idt.interrupts[129].set_handler_fn(keyboard_handler) ;
// idt.interrupts[130].set_handler_fn(keyboard_handler) ;
// idt.interrupts[131].set_handler_fn(keyboard_handler) ;
// idt.interrupts[132].set_handler_fn(keyboard_handler) ;
// idt.interrupts[133].set_handler_fn(keyboard_handler) ;
// idt.interrupts[134].set_handler_fn(keyboard_handler) ;
// idt.interrupts[135].set_handler_fn(keyboard_handler) ;
// idt.interrupts[136].set_handler_fn(keyboard_handler) ;
// idt.interrupts[137].set_handler_fn(keyboard_handler) ;
// idt.interrupts[138].set_handler_fn(keyboard_handler) ;
// idt.interrupts[139].set_handler_fn(keyboard_handler) ;
// idt.interrupts[140].set_handler_fn(keyboard_handler) ;
// idt.interrupts[141].set_handler_fn(keyboard_handler) ;
// idt.interrupts[142].set_handler_fn(keyboard_handler) ;
// idt.interrupts[143].set_handler_fn(keyboard_handler) ;
// idt.interrupts[144].set_handler_fn(keyboard_handler) ;
// idt.interrupts[145].set_handler_fn(keyboard_handler) ;
// idt.interrupts[146].set_handler_fn(keyboard_handler) ;
// idt.interrupts[147].set_handler_fn(keyboard_handler) ;
// idt.interrupts[148].set_handler_fn(keyboard_handler) ;
// idt.interrupts[149].set_handler_fn(keyboard_handler) ;
// idt.interrupts[150].set_handler_fn(keyboard_handler) ;
// idt.interrupts[151].set_handler_fn(keyboard_handler) ;
// idt.interrupts[152].set_handler_fn(keyboard_handler) ;
// idt.interrupts[153].set_handler_fn(keyboard_handler) ;
// idt.interrupts[154].set_handler_fn(keyboard_handler) ;
// idt.interrupts[155].set_handler_fn(keyboard_handler) ;
// idt.interrupts[156].set_handler_fn(keyboard_handler) ;
// idt.interrupts[157].set_handler_fn(keyboard_handler) ;
// idt.interrupts[158].set_handler_fn(keyboard_handler) ;
// idt.interrupts[159].set_handler_fn(keyboard_handler) ;
// idt.interrupts[160].set_handler_fn(keyboard_handler) ;
// idt.interrupts[161].set_handler_fn(keyboard_handler) ;
// idt.interrupts[162].set_handler_fn(keyboard_handler) ;
// idt.interrupts[163].set_handler_fn(keyboard_handler) ;
// idt.interrupts[164].set_handler_fn(keyboard_handler) ;
// idt.interrupts[165].set_handler_fn(keyboard_handler) ;
// idt.interrupts[166].set_handler_fn(keyboard_handler) ;
// idt.interrupts[167].set_handler_fn(keyboard_handler) ;
// idt.interrupts[168].set_handler_fn(keyboard_handler) ;
// idt.interrupts[169].set_handler_fn(keyboard_handler) ;
// idt.interrupts[170].set_handler_fn(keyboard_handler) ;
// idt.interrupts[171].set_handler_fn(keyboard_handler) ;
// idt.interrupts[172].set_handler_fn(keyboard_handler) ;
// idt.interrupts[173].set_handler_fn(keyboard_handler) ;
// idt.interrupts[174].set_handler_fn(keyboard_handler) ;
// idt.interrupts[175].set_handler_fn(keyboard_handler) ;
// idt.interrupts[176].set_handler_fn(keyboard_handler) ;
// idt.interrupts[177].set_handler_fn(keyboard_handler) ;
// idt.interrupts[178].set_handler_fn(keyboard_handler) ;
// idt.interrupts[179].set_handler_fn(keyboard_handler) ;
// idt.interrupts[180].set_handler_fn(keyboard_handler) ;
// idt.interrupts[181].set_handler_fn(keyboard_handler) ;
// idt.interrupts[182].set_handler_fn(keyboard_handler) ;
// idt.interrupts[183].set_handler_fn(keyboard_handler) ;
// idt.interrupts[184].set_handler_fn(keyboard_handler) ;
// idt.interrupts[185].set_handler_fn(keyboard_handler) ;
// idt.interrupts[186].set_handler_fn(keyboard_handler) ;
// idt.interrupts[187].set_handler_fn(keyboard_handler) ;
// idt.interrupts[188].set_handler_fn(keyboard_handler) ;
// idt.interrupts[189].set_handler_fn(keyboard_handler) ;
// idt.interrupts[190].set_handler_fn(keyboard_handler) ;
// idt.interrupts[191].set_handler_fn(keyboard_handler) ;
// idt.interrupts[192].set_handler_fn(keyboard_handler) ;
// idt.interrupts[193].set_handler_fn(keyboard_handler) ;
// idt.interrupts[194].set_handler_fn(keyboard_handler) ;
// idt.interrupts[195].set_handler_fn(keyboard_handler) ;
// idt.interrupts[196].set_handler_fn(keyboard_handler) ;
// idt.interrupts[197].set_handler_fn(keyboard_handler) ;
// idt.interrupts[198].set_handler_fn(keyboard_handler) ;
// idt.interrupts[199].set_handler_fn(keyboard_handler) ;
// idt.interrupts[200].set_handler_fn(keyboard_handler) ;
// idt.interrupts[201].set_handler_fn(keyboard_handler) ;
// idt.interrupts[202].set_handler_fn(keyboard_handler) ;
// idt.interrupts[203].set_handler_fn(keyboard_handler) ;
// idt.interrupts[204].set_handler_fn(keyboard_handler) ;
// idt.interrupts[205].set_handler_fn(keyboard_handler) ;
// idt.interrupts[206].set_handler_fn(keyboard_handler) ;
// idt.interrupts[207].set_handler_fn(keyboard_handler) ;
// idt.interrupts[208].set_handler_fn(keyboard_handler) ;
// idt.interrupts[209].set_handler_fn(keyboard_handler) ;
// idt.interrupts[210].set_handler_fn(keyboard_handler) ;
// idt.interrupts[211].set_handler_fn(keyboard_handler) ;
// idt.interrupts[212].set_handler_fn(keyboard_handler) ;
// idt.interrupts[213].set_handler_fn(keyboard_handler) ;
// idt.interrupts[214].set_handler_fn(keyboard_handler) ;
// idt.interrupts[215].set_handler_fn(keyboard_handler) ;
// idt.interrupts[216].set_handler_fn(keyboard_handler) ;
// idt.interrupts[217].set_handler_fn(keyboard_handler) ;
// idt.interrupts[218].set_handler_fn(keyboard_handler) ;
// idt.interrupts[219].set_handler_fn(keyboard_handler) ;
// idt.interrupts[220].set_handler_fn(keyboard_handler) ;
// idt.interrupts[221].set_handler_fn(keyboard_handler) ;
// idt.interrupts[222].set_handler_fn(keyboard_handler) ;
// idt.interrupts[223].set_handler_fn(keyboard_handler) ;

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

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut ExceptionStackFrame)
{
    //println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
extern "x86-interrupt" fn timer_handler(stack_frame: &mut ExceptionStackFrame)
{
    // println!("Timer Tik-tok Tik-tok!!!");
    unsafe { chainedPics.ChainedPics_sendEOI(0) ; }
}
extern "x86-interrupt" fn keyboard_handler(stack_frame: &mut ExceptionStackFrame)
{
    //println!("A key is pressed!!!");
    println!("The key is := {:?}\n", Port::new(0x60).in8()) ;
    Port::new(0x60).in8() ; 
    unsafe { chainedPics.ChainedPics_sendEOI(1) ; }

}
extern "x86-interrupt" fn nic_interrupt_handler(stack_frame: &mut ExceptionStackFrame)
{
    println!("A packet is received!!!");
}
extern "x86-interrupt" fn double_fault_handler(
    stack_sframe: &mut ExceptionStackFrame, _error_code: u64)
{
    println!("\nEXCEPTION: DOUBLE FAULT\n{:#?}", stack_sframe);
    loop {}
}
