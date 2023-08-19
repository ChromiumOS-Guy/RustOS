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
mod PageTableManager;
mod BasicRenderer;
mod Rstr;

use BasicRenderer::Next;
use BasicRenderer::Print;
use PageTableManager::PageFrameAllocator;
use PageTableManager::paging;
use std::arch::asm;

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

extern "C" {
    pub fn Mov2CR3(pml4: *mut paging::PageTable);
}

// main logic
#[no_mangle]
extern "C" fn _start(bootInfo:*mut BootInfo) {
    // Init libs
    BasicRenderer::BasicRenderer(unsafe {&mut *(*bootInfo).framebuffer}, unsafe {&mut *(*bootInfo).psf1_Font});
    unsafe {PageFrameAllocator::ReadEFIMemoryMap((*bootInfo).mMap, (*bootInfo).mMapSize, (*bootInfo).mMapDescSize)};
    // make MemoryMap Page Table
    let page = unsafe {PageFrameAllocator::RequestPage()};
    for i in 0..0x1000 { // make sure allocated page is empty
        unsafe {
            *(page.offset(i as isize)) = std::mem::transmute(0u8);
        }
    }
    let mut PML4 : *mut paging::PageTable = page as *mut paging::PageTable;
    PageTableManager::PageTableManager(PML4);
    // init PageTableManager
    let mut current = 0;
    while current < unsafe {PageFrameAllocator::EfiMemory::get_memory_size((*bootInfo).mMap , (*bootInfo).mMapSize / (*bootInfo).mMapDescSize, (*bootInfo).mMapDescSize)} {
        PageTableManager::MapMemory(current as *mut std::ffi::c_void, current as *mut std::ffi::c_void);
        current += 0x1000;
    }
    // make sure frame buffer is allocated memory
    current = 0;
    // we add + 0x1000 to make sure we don't write to little memory for the framebuffer to use.
    while current < unsafe {(*(*bootInfo).framebuffer).BaseAddress as usize + (*(*bootInfo).framebuffer).BufferSize + 0x1000usize}{
        PageTableManager::MapMemory(current as *mut std::ffi::c_void, current as *mut std::ffi::c_void);
        current += 4096; // page size
    }

    // move PML4 to CR3 register
    //unsafe {asm!("mov {}, cr3", in(reg) PML4)}; //not working for some reason bigger problems rn so i just passed it to C++ but I will fix it
    unsafe {Mov2CR3(PML4)};
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
    Print(r"1. Need to Implement some sort of mem PageTableManager");
    Next();
    Print(r"2. IDT I need it so I can do shit with PIT when I make IO lib");
    Next();
    Next();
    Print(r"RAM_INFO:");
    Next();
    Print(r"FreeRAM: ");
    Print(&Rstr::usize_to_str(PageFrameAllocator::GetFreeRAM()));
    Next();
    Print(r"UsedRAM: ");
    Print(&Rstr::usize_to_str(PageFrameAllocator::GetUsedRAM()));
    Next();
    Print(r"ReservedRAM: ");
    Print(&Rstr::usize_to_str(PageFrameAllocator::GetReservedRAM()));
    Next();

    for __i in 0..16{
        Next();
    }
    Print(r"Initiating Grahpics Test with GOP (No Double buffer): ");
    loop {
        let mut x = 0;
        let mut y = unsafe {(*(*bootInfo).framebuffer).Height} / 2;
        for i in 0..2147483647u32 {
            BasicRenderer::PutPix(x , y , i);
            if x < unsafe {(*(*bootInfo).framebuffer).Width} / 2{
                // logic for passing on every pixel in screen
                x += 1;
            } else if y < unsafe {(*(*bootInfo).framebuffer).Height} {
                y += 1;
                x = 0;
            } else if y >= unsafe {(*(*bootInfo).framebuffer).Height} / 2{
                x = 0;
                y = unsafe {(*(*bootInfo).framebuffer).Height} / 2;
            }
        }

    }
}