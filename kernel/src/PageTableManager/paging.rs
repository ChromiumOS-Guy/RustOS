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

#[derive(Copy, Clone)]
#[repr(C)]
pub enum PT_Flag {
    Present = 0,
    ReadWrite = 1,
    UserSuper = 2,
    WriteThrough = 3,
    CacheDisabled = 4,
    Accessed = 5,
    LargerPages = 7,
    Custom0 = 9,
    Custom1 = 10,
    Custom2 = 11,
    NX = 63 
}
#[repr(C)]
pub struct PageTable {
    pub entries: [PageDirectoryEntry; 512],
}

#[repr(C)]
#[repr(align(0x1000))]
pub struct PageDirectoryEntry {
	pub value : u64
}

impl PageDirectoryEntry {
    pub fn SetFlag(&mut self, flag: PT_Flag, enabled: bool) {
        let bit_selector: u64 = 1 << flag as std::ffi::c_ulong;
        self.value &= !bit_selector;
        if enabled {
            self.value |= bit_selector;
        }
    }

    pub fn GetFlag(&self, flag: PT_Flag) -> bool {
        let bit_selector: u64 = 1 << flag as std::ffi::c_ulong;
        self.value & bit_selector > 0
    }

    pub fn GetAddress(&self) -> u64 {
        (self.value & 0x000ffffffffff000) >> 12
    }

    pub fn SetAddress(&mut self, address: u64) {
        let address = address & 0x000000ffffffffff;
        self.value &= 0xfff0000000000fff;
        self.value |= address << 12;
    }
}

impl Copy for PageDirectoryEntry { }

impl Clone for PageDirectoryEntry {
    fn clone(&self) -> PageDirectoryEntry {
        *self
    }
}