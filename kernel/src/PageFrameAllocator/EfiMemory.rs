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

#[repr(C)]
pub struct EFI_MEMORY_DESCRIPTOR {
    // EfiMemory Decriptor of Memory Entries
    pub r#type_: std::ffi::c_uint,
    pub phys_addr: *mut std::ffi::c_void,
    pub virt_addr: *mut std::ffi::c_void,
    pub num_pages: usize,
    pub attribs: std::ffi::c_ulonglong,
}


pub const EFI_MEMORY_TYPE_STRINGS: [&str; 14] = [
    // EfiMemory Types
    "EfiReservedMemoryType",
    "EfiLoaderCode",
    "EfiLoaderData",
    "EfiBootServicesCode",
    "EfiBootServicesData",
    "EfiRuntimeServicesCode",
    "EfiRuntimeServicesData",
    "EfiConventionalMemory",
    "EfiUnusableMemory",
    "EfiACPIReclaimMemory",
    "EfiACPIMemoryNVS",
    "EfiMemoryMappedIO",
    "EfiMemoryMappedIOPortSpace",
    "EfiPalCode",
];

// EfiMemory

extern "C" {
    pub fn GetMemoryDesc(mMap: *mut EFI_MEMORY_DESCRIPTOR, current: usize, mMapDescSize: usize) -> *mut EFI_MEMORY_DESCRIPTOR;
}

pub fn get_memory_size( // gets stuck in for loop
    // gets total size of memory
    mMap: *mut EFI_MEMORY_DESCRIPTOR,
    mMapEntries: usize,
    mMapDescSize : usize,
) -> usize {
    let mut memory_size = 0;

    let mut current : usize = 0;
    unsafe {while current < mMapEntries {
        let desc = &*GetMemoryDesc(mMap , current, mMapDescSize);
        memory_size += desc.num_pages * 4096;
        current += 1;
    }}

    return memory_size;
}



#[inline]
pub fn memset(start: *mut std::ffi::c_void, value: u8, num: u64) {
    // memset need i say more about it its useful as the word useful can get
    for i in 0..num {
        unsafe {
            *(start.offset(i as isize)) = std::mem::transmute(value);
        }
    }
}
