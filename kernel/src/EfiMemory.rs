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

pub struct EFI_MEMORY_DESCRIPTOR {
    // EfiMemory Decriptor of Memory Entries
    pub r#type_: std::ffi::c_uint,
    pub phys_addr: *mut u64,
    pub virt_addr: *mut u64,
    pub num_pages: std::ffi::c_ulonglong,
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
pub fn get_memory_size(
    // gets total size of memory
    mMap: *const EFI_MEMORY_DESCRIPTOR,
    mMapEntries: usize,
) -> u64 {
    let mut memory_size = 0;

    let mut current = 0;
    while current < mMapEntries {
        let desc = unsafe { &*mMap.add(current as usize) };
        if desc.type_ != 0 {
            memory_size += desc.num_pages * 4096;
        }
        current += 1;
    }

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
