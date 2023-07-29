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
mod EfiMemory;
mod Rstr;
use BasicRenderer::Next;
use BasicRenderer::Print;

//structs
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BootInfo {
    // BootInfo contains boot data such as GOP ,font , EfiMemory ,etc...
    pub framebuffer: *mut BasicRenderer::Framebuffer,
    pub psf1_Font: *mut BasicRenderer::PSF1_FONT,
    pub mMap: *mut EfiMemory::EFI_MEMORY_DESCRIPTOR,
    pub mMapSize: usize,
    pub mMapDescSize: usize,
}


// main logic
#[no_mangle]
extern "C" fn _start(bootInfo:*mut BootInfo) {
    // EfiMemory stuff
    let mMapEntries = unsafe {(*bootInfo).mMapSize / (*bootInfo).mMapDescSize};
    let mut memory_size = EfiMemory::get_memory_size(unsafe {(*bootInfo).mMap}, mMapEntries);
    EfiMemory::memset(unsafe {&mut *(*bootInfo).framebuffer}.BaseAddress , 0 ,unsafe {&mut *(*bootInfo).framebuffer}.BufferSize); // Clear framebuffer from bootloader log

    //Render init
    BasicRenderer::BasicRenderer(unsafe {&mut *(*bootInfo).framebuffer}, unsafe {&mut *(*bootInfo).psf1_Font});
    // logic
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
    Print(r"1. convert all EfiMemory & Bitmap to use u64 instead of usize to prevent current data loss");
    Next();
    Print(r"2. Need to Implement some sort of mem PageManager");
    Next();
    Print(r"3. IDT I need it so I can do shit with PIT when I make IO lib");
    Next();
    Next();
    Print(r"INFO:");
    Next();
    Print(r"Total Memory Size is: ");
    Print(&Rstr::u64_to_str(memory_size));
    Print(r" Bytes");
    Next();
    Print(r"Memory Map Entries: "); // Displays current relevant mMapEntries by printing the mMapEntries names
    Print(&Rstr::usize_to_str(mMapEntries));
    unsafe {
        BasicRenderer::CursorPosition.Y = 0; // dodgy shit with CursorPosition to get EFI Memory Map out of the way of INFO (only use Next() if you can)
        BasicRenderer::CursorPosition.X = 1400;
    }
    Print(r"EFI Memory Map:"); 
    let mut current = 0;
    while current < mMapEntries {
        let desc = unsafe { &*(*bootInfo).mMap.add(current as usize) };
        if desc.type_ != 0 {
            let mut memory_type_string = match EfiMemory::EFI_MEMORY_TYPE_STRINGS.get(desc.type_ as usize) {
                Some(string) => string,
                None => "UnknownEntryType",
            };
            unsafe {
                BasicRenderer::CursorPosition.Y = BasicRenderer::CursorPosition.Y + 16; // same dodgy shit as before
                BasicRenderer::CursorPosition.X = 1400;
            }
            BasicRenderer::Colour(0x0FFF00u32);
            Print(memory_type_string);
            Print(" "); // still need to find a way to Print fucking numbers to the screen fuck why is rust so hard
            BasicRenderer::Colour(0xFFFF00FFu32);
            Print(&Rstr::u64_to_str(desc.num_pages * 4096 / 1024));
            BasicRenderer::Colour(0x0FFF00u32);
            Print(" KB");
        }
        current += 1;
    }
    loop {

    }
}