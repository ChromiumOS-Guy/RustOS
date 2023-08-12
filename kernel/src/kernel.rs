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
// modules
mod BasicRenderer;
mod Rstr;
mod PageFrameAllocator;
use BasicRenderer::Next;
use BasicRenderer::Print;

//structs
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BootInfo {
    // BootInfo contains boot data such as GOP ,font , EfiMemory ,etc...
    pub framebuffer: *mut BasicRenderer::Framebuffer,
    pub psf1_Font: *mut BasicRenderer::PSF1_FONT,
    pub mMap: *mut PageFrameAllocator::EfiMemory::EFI_MEMORY_DESCRIPTOR,
    pub mMapSize: usize,
    pub mMapDescSize: usize,
}


// main logic
#[no_mangle]
extern "C" fn _start(bootInfo:*mut BootInfo) {
    // Init libs
    BasicRenderer::BasicRenderer(unsafe {&mut *(*bootInfo).framebuffer}, unsafe {&mut *(*bootInfo).psf1_Font});
    unsafe {PageFrameAllocator::ReadEFIMemoryMap((*bootInfo).mMap, (*bootInfo).mMapSize, (*bootInfo).mMapDescSize)};

    // user INFO code:
    BasicRenderer::Clear(0x000000u32); // clean screen
    BasicRenderer::Colour(0xFF0000u32);
    Print(r"  _____           _    ____   _____"); // ASCII art
    Next();
    Print(r" |  __ \         | |  / __ \ / ____|");
    Next();
    Print(r" | |__) |   _ ___| |_| |  | | (___  ");
    Next();
    Print(r" |  _  / | | / __| __| |  | |\___ \ ");
    Next();
    Print(r" | | \ \ |_| \__ \ |_| |__| |____) |");
    Next();
    Print(r" |_|  \_\__,_|___/\__|\____/|_____/ ");
    Next();
    Next();
    Next();
    BasicRenderer::Colour(0xFFFFFFu32);
    Print(r"To Do:");
    Next();
    Print(r"1. Need to Implement some sort of mem PageManager");
    Next();
    Print(r"2. IDT I need it so I can do shit with PIT when I make IO lib");
    Next();
    Next();
    Print("Testing Rstr should be 100:   ");
    Print(&Rstr::u8_to_str(100u8));
    Next();
    Print(r"RAM_INFO:");
    Next();
    Print(r"freeMemory: ");
    Print(&Rstr::usize_to_str(PageFrameAllocator::GetFreeRAM()));
    Next();
    Print(r"usedMemory: ");
    Print(&Rstr::usize_to_str(PageFrameAllocator::GetUsedRAM()));
    Next();
    Print(r"reservedMemory: ");
    Print(&Rstr::usize_to_str(PageFrameAllocator::GetReservedRAM()));
    Next();
    Print("Requesting 20 Pages in Memory: ");
    for i in 0..20{
        Next();
        let address : *mut std::ffi::c_void = unsafe {PageFrameAllocator::RequestPage()};
        Print(&Rstr::u64_to_str(address as u64));


    }
    Next();
    Print("usedMemory: ");
    Print(&Rstr::usize_to_str(PageFrameAllocator::GetUsedRAM()));
    loop {

    }
}