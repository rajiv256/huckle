pub mod area_frame_allocator ; 

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}
pub const PAGE_SIZE: usize = 4096;

use ::paging::PhysicalAddress ; 

impl Frame {
    pub fn containing_address(address: usize) -> Frame {
        Frame{ number: address / PAGE_SIZE }
    }
    
	pub fn start_address(&self) -> ::paging::PhysicalAddress{
	    self.number * PAGE_SIZE
	}
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

