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
use std::convert::TryInto;

static mut freeMemory : u64 = 0;
static mut reservedMemory : u64 = 0;
static mut usedMemory : u64 = 0;
static mut Initialized : bool = false;
static mut PageBitmap : *mut Bitmap::Bitmap = std::ptr::null_mut();


pub unsafe fn ReadEFIMemoryMap(mMap: *mut EfiMemory::EFI_MEMORY_DESCRIPTOR, mMapSize: u64, mMapDescSize: u64) {
    if Initialized {return};
    Initialized = true;

    let mut largsetFreeMemSeg: *mut u64 = std::ptr::null_mut();
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

    // Initialize bitmap
    InitBitmap(bitmapSize.try_into().unwrap(), largsetFreeMemSeg);

    // lock pages where bitmap is (which is largsetFreeMemSeg)
    LockPages(largsetFreeMemSeg, ((*PageBitmap).size / 4096 + 1).try_into().unwrap());
    // reserve pages of unusable/reserved memory
    current = 0;
    while current < mMapEntries {
        let desc = &*mMap.add(current as usize);
        if desc.type_ != 7 { // not EfiConventionalMemory
            ReservePages(desc.phys_addr , desc.num_pages);
        }
    }
}

// private
unsafe fn InitBitmap(bitmap_size: usize, buffer_address: *mut u64) { // dose not work
    // Set the size of the bitmap.
    (*PageBitmap).size = bitmap_size;

    // Set the buffer address of the bitmap.
    //(*PageBitmap).buffer = pointer to buffer_address (still don't know how to do)

    // Initialize the bitmap.
    for i in 0..bitmap_size {
        (*PageBitmap).buffer[i] = 0;
    }
}

unsafe fn UnreservePage(address: *mut u64) {
    let index = (address as u64) / 4096;
    if !(*PageBitmap).get(index.try_into().unwrap()){
        return;
    }
    if (*PageBitmap).set(index.try_into().unwrap(), false){
        freeMemory += 4096;
        reservedMemory -= 4096;
    }
}

unsafe fn ReservePage(address: *mut u64) {
    let index = (address as u64) / 4096;
    if (*PageBitmap).get(index.try_into().unwrap()){
        return;
    }
    if (*PageBitmap).set(index.try_into().unwrap(), true){
        freeMemory -= 4096;
        reservedMemory += 4096;
    }
}

unsafe fn UnreservePages(address: *mut u64, pageCount: u64) {
    for t in 0..pageCount {
        let pageAddress = (address as u64) + (t * 4096);
        UnreservePage(pageAddress as *mut u64);
    }
}

unsafe fn ReservePages(address: *mut u64, pageCount: u64) {
    for t in 0..pageCount {
        let pageAddress = (address as u64) + (t * 4096);
        ReservePage(pageAddress as *mut u64);
    }
}

// public

pub unsafe fn FreePage(address: *mut u64) {
    let index = (address as u64) / 4096;
    if !(*PageBitmap).get(index.try_into().unwrap()){
        return;
    }
    if (*PageBitmap).set(index.try_into().unwrap(), false){
        freeMemory += 4096;
        usedMemory -= 4096;
    }
}

pub unsafe fn LockPage(address: *mut u64) {
    let index = (address as u64) / 4096;
    if (*PageBitmap).get(index.try_into().unwrap()){
        return;
    }
    if (*PageBitmap).set(index.try_into().unwrap(), true){
        freeMemory -= 4096;
        usedMemory += 4096;
    }
}

pub unsafe fn FreePages(address: *mut u64, pageCount: u64) {
    for t in 0..pageCount {
        let pageAddress = (address as u64) + (t * 4096);
        FreePage(pageAddress as *mut u64);
    }
}

pub unsafe fn LockPages(address: *mut u64, pageCount: u64) {
    for t in 0..pageCount {
        let pageAddress = (address as u64) + (t * 4096);
        LockPage(pageAddress as *mut u64);
    }
}

pub unsafe fn GetFreeRAM() -> u64 {
    return freeMemory;
}

pub unsafe fn GetUsedRAM() -> u64 {
    return usedMemory;
}

pub unsafe fn GetReservedRAM() -> u64 {
    return reservedMemory;
}