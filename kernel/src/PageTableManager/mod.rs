#![allow(
    dead_code,
    unused_variables,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub mod PageFrameAllocator;
pub mod paging;


// Indexing
pub fn page_map_indexer(mut virtual_address: usize) -> (usize, usize, usize, usize) { // Input Address -> 48 bits
  									   //                   +--> Page offset: bits [11:0]
  virtual_address >>= 12;            
  let P_i = virtual_address & 0x1ff;   //                +--> Level 3: bits [20:12] (Page)
  virtual_address >>= 9; 
  let PT_i = virtual_address & 0x1ff;  //            +--> Level 2: bits [29:21] (Page Table)
  virtual_address >>= 9;
  let PD_i = virtual_address & 0x1ff;  //        +--> Level 1: bits [38:30] (Page Dir Table)
  virtual_address >>= 9;
  let PDP_i = virtual_address & 0x1ff; //    +--> Level 0: bits [47:39] (Page Dir Pointer Table)

  (PDP_i, PD_i, PT_i, P_i)
} // Page Map full size is 512^4 for each level ^1

// PageTableManager
static mut PML4 : *mut paging::PageTable = std::ptr::null_mut();

pub fn PageTableManager(PML4Address: *mut paging::PageTable){
    unsafe {PML4 = PML4Address};
}

pub fn MapMemory(virtualmemory: *mut std::ffi::c_void, physicalmemory: *mut std::ffi::c_void) -> bool {
	if !PageFrameAllocator::Is_Initialized() {
		return false
	}
	let indexer = page_map_indexer(virtualmemory as usize);
	let mut PDE : paging::PageDirectoryEntry = unsafe {(*PML4).entries[indexer.0]};
	
	let mut PDP : *mut paging::PageTable = std::ptr::null_mut();
	if !PDE.GetFlag(paging::PT_Flag::Present){ // getting the PDP or making it if it doesn't exists
		let page = unsafe {PageFrameAllocator::RequestPage()};
		for i in 0..0x1000 { // make sure allocated page is empty
	        unsafe {
	            *(page.offset(i as isize)) = std::mem::transmute(0u8);
	        }
	    }
		PDP = page as *mut paging::PageTable;
		PDE.SetAddress(PDP as u64 >> 12);
		PDE.SetFlag(paging::PT_Flag::Present, true);
		PDE.SetFlag(paging::PT_Flag::ReadWrite, true);
		unsafe {(*PML4).entries[indexer.0] = PDE};
	} else {
		PDP = (PDE.GetAddress() << 12) as *mut paging::PageTable;
	}

	PDE = unsafe {(*PDP).entries[indexer.1]};
	let mut PD : *mut paging::PageTable = std::ptr::null_mut();
	if !PDE.GetFlag(paging::PT_Flag::Present){ // getting the PD or making it if it doesn't exists
		let page = unsafe {PageFrameAllocator::RequestPage()};
		for i in 0..0x1000 { // make sure allocated page is empty
	        unsafe {
	            *(page.offset(i as isize)) = std::mem::transmute(0u8);
	        }
	    }
		PD = page as *mut paging::PageTable;
		PDE.SetAddress(PD as u64 >> 12);
		PDE.SetFlag(paging::PT_Flag::Present, true);
		PDE.SetFlag(paging::PT_Flag::ReadWrite, true);
		unsafe {(*PDP).entries[indexer.1] = PDE};
	} else {
		PD = (PDE.GetAddress() << 12) as *mut paging::PageTable;
	}

	PDE = unsafe {(*PD).entries[indexer.2]};
	let mut PT : *mut paging::PageTable = std::ptr::null_mut();
	if !PDE.GetFlag(paging::PT_Flag::Present){ // getting the PT or making it if it doesn't exists
		let page = unsafe {PageFrameAllocator::RequestPage()};
		for i in 0..0x1000 { // make sure allocated page is empty
			unsafe {
	            *(page.offset(i as isize)) = std::mem::transmute(0u8);
	        }
		}
		PT = page as *mut paging::PageTable;
		PDE.SetAddress(PT as u64 >> 12);
		PDE.SetFlag(paging::PT_Flag::Present, true);
		PDE.SetFlag(paging::PT_Flag::ReadWrite, true);
		unsafe {(*PD).entries[indexer.2] = PDE};
	} else {
		PT = (PDE.GetAddress() << 12) as *mut paging::PageTable;
	}

	PDE = unsafe {(*PT).entries[indexer.3]};
	PDE.SetAddress(physicalmemory as u64 >> 12);
	PDE.SetFlag(paging::PT_Flag::Present, true);
	PDE.SetFlag(paging::PT_Flag::ReadWrite, true);
	unsafe {(*PT).entries[indexer.3] = PDE};
	return true;
}