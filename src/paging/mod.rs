pub mod entry ; 
pub mod table ; 

pub use self::entry::*;
use ::memory::FrameAllocator;

use self::table::{Table, Level4};
use core::ptr::Unique;

const ENTRY_COUNT :usize  = 512 ; 
use ::memory::PAGE_SIZE ; 
use ::memory::Frame ; 

pub type PhysicalAddress = usize;
pub type VirtualAddress = usize;

pub struct Page {
   number: usize,
}

pub struct ActivePageTable {
    p4: Unique<Table<Level4>>,
}
impl ActivePageTable {
    pub unsafe fn new() -> ActivePageTable {
        ActivePageTable {
            p4: Unique::new(table::P4),
        }
    }
    fn p4(&self) -> &Table<Level4> {
	    unsafe { self.p4.get() }
	}

	fn p4_mut(&mut self) -> &mut Table<Level4> {
	    unsafe { self.p4.get_mut() }
	}

}










