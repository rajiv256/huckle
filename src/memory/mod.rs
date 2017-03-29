pub mod area_frame_allocator ; 
pub mod paging ; 
pub const PAGE_SIZE: usize = 4096;

use self::paging::PhysicalAddress ; 
pub use self::paging::test_paging;
pub use self::paging::remap_the_kernel;

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

	fn clone(&self) -> Frame {
        Frame { number: self.number }
    }
}

impl Frame {
    fn range_inclusive(start: Frame, end: Frame) -> FrameIter {
        FrameIter {
            start: start,
            end: end,
        }
    }
}

struct FrameIter {
    start: Frame,
    end: Frame,
}

impl Iterator for FrameIter {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        if self.start <= self.end {
            let frame = self.start.clone();
            self.start.number += 1;
            Some(frame)
        } else {
            None
        }
    }
 }

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

