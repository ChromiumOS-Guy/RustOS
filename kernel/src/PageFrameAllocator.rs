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
mod EfiMemory;
mod Bitmap;

static mut freeMemory : u64 = 0;
static mut reservedMemory : u64 = 0;
static mut usedMemory : u64 = 0;
static mut Initialized : bool = false;
static mut PageBitmap : *mut Bitmap::Bitmap = std::ptr::null_mut();

pub unsafe fn ReadEFIMemoryMap(mMap: *mut EfiMemory::EFI_MEMORY_DESCRIPTOR, mMapSize: usize, mMapDescSize: usize) {
    if Initialized {return};
    Initialized = true;

    let mut largsetFreeMemSeg : *mut u64 = std::ptr::null_mut();
    let mut largsetFreeMemSegSize : u64 = 0;

    let mMapEntries = mMapSize / mMapDescSize;
    let mut current = 0;
    while current < mMapEntries {
        let desc = &*mMap.add(current as usize);
        if desc.type_ == 7 { // type 7 is EfiConventionalMemory
            if desc.num_pages * 4096 > largsetFreeMemSegSize {
                largsetFreeMemSeg = desc.phys_addr;
                largsetFreeMemSegSize = desc.num_pages * 4096;
            }
        }
        current += 1;
    }
    let mut memory_size : u64 = EfiMemory::get_memory_size(mMap, mMapEntries);
    freeMemory = memory_size;
    let bitmapSize = memory_size / 4096 / 8 + 1;

    //Initialize bitmap



}

// pub fn InitBitmap(bitmapSize : u64 , BufferAddress : *mut u64) {
//     PageBitmap = Bitmap::Bitmap::new(bitmapSize);
// }