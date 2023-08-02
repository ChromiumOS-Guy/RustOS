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
pub mod EfiMemory;
mod Bitmap;

static mut freeMemory : usize = 0;
static mut reservedMemory : usize = 0;
static mut usedMemory : usize = 0;
static mut Initialized : bool = false;
static mut PageBitmap: Bitmap::Bitmap = Bitmap::Bitmap::new_placeholder();


pub unsafe fn ReadEFIMemoryMap(mMap: *mut EfiMemory::EFI_MEMORY_DESCRIPTOR, mMapSize: usize, mMapDescSize: usize) {
    if Initialized {return};
    Initialized = true;

    let mut largsetFreeMemSeg: *mut std::ffi::c_void = std::ptr::null_mut();
    let mut largsetFreeMemSegSize : usize = 0;

    let mMapEntries = mMapSize / mMapDescSize;
    let mut current = 0;
    while current < mMapEntries {
        let desc = unsafe { &*mMap.add(current * mMapDescSize)};
        if desc.type_ == 7 { // type 7 is EfiConventionalMemory
            if desc.num_pages * 4096 > largsetFreeMemSegSize {
                largsetFreeMemSeg = desc.phys_addr;
                largsetFreeMemSegSize = desc.num_pages * 4096;
            }
        }
        current += 1;
    }
    let mut memory_size : usize = EfiMemory::get_memory_size(mMap, mMapEntries, mMapDescSize);
    freeMemory = memory_size;
    let bitmapSize = memory_size / 4096 / 8 + 1;

    // Initialize bitmap
    PageBitmap = Bitmap::Bitmap::new(bitmapSize, largsetFreeMemSeg);
    // lock pages where bitmap is (which is largsetFreeMemSeg)
    LockPages(largsetFreeMemSeg, PageBitmap.size / 4096 + 1); // lockPages gets stuck (current suspicion is that get_memory_size fn isn't returning true size of memory)
    // reserve pages of unusable/reserved memory
    current = 0;
    while current < mMapEntries {
        let desc = unsafe { &*mMap.add(current * mMapDescSize)};
        if desc.type_ != 7 { // not EfiConventionalMemory
            ReservePages(desc.phys_addr , desc.num_pages);
        }
    }
}

// private
unsafe fn UnreservePage(address: *mut std::ffi::c_void) {
    let index = (address as usize) / 4096;
    if !PageBitmap.get(index){
        return;
    }
    if PageBitmap.set(index, false){
        freeMemory += 4096;
        reservedMemory -= 4096;
    }
}

unsafe fn ReservePage(address: *mut std::ffi::c_void) {
    let index = (address as usize) / 4096;
    if PageBitmap.get(index){
        return;
    }
    if PageBitmap.set(index, true){
        freeMemory -= 4096;
        reservedMemory += 4096;
    }
}

unsafe fn UnreservePages(address: *mut std::ffi::c_void, pageCount: usize) {
    for t in 0..pageCount {
        UnreservePage(address.add(t * 4096));
    }
}

unsafe fn ReservePages(address: *mut std::ffi::c_void, pageCount: usize) {
    for t in 0..pageCount {
        ReservePage(address.add(t * 4096));
    }
}

// public

pub unsafe fn FreePage(address: *mut std::ffi::c_void) {
    let index = (address as usize) / 4096;
    if !PageBitmap.get(index){
        return;
    }
    if PageBitmap.set(index, false){
        freeMemory += 4096;
        usedMemory -= 4096;
    }
}

pub unsafe fn LockPage(address: *mut std::ffi::c_void) {
    let index = (address as usize) / 4096;
    if PageBitmap.get(index){
        return;
    }
    if PageBitmap.set(index, true){
        freeMemory -= 4096;
        usedMemory += 4096;
    }
}

pub unsafe fn FreePages(address: *mut std::ffi::c_void, pageCount: usize) {
    for t in 0..pageCount {
        FreePage(address.add(t * 4096));
    }
}

pub unsafe fn LockPages(address: *mut std::ffi::c_void, pageCount: usize) {
    for t in 0..pageCount {
        LockPage(address.add(t * 4096));
    }
}

pub fn GetFreeRAM() -> usize {
    return unsafe {freeMemory};
}

pub fn GetUsedRAM() -> usize {
    return unsafe {usedMemory};
}

pub fn GetReservedRAM() -> usize {
    return unsafe {reservedMemory};
}