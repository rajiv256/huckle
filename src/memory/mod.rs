pub mod area_frame_allocator ; 
pub mod paging ; 
pub const PAGE_SIZE: usize = 4096;

use self::paging::PhysicalAddress ; 
pub use self::paging::test_paging;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

impl Frame {
    pub fn containing_address(address: usize) -> Frame {
        Frame{ number: address / PAGE_SIZE }
    }
    
	pub fn start_address(&self) -> self::paging::PhysicalAddress{
	    self.number * PAGE_SIZE
	}
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

