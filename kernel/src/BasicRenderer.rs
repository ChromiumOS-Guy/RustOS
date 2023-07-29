#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use std::convert::TryInto;
use std::ptr;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Framebuffer {
    // framebuffer has resolution data and GOP addresses
    pub BaseAddress: *mut std::ffi::c_void,
    pub BufferSize: std::ffi::c_ulonglong,
    pub Width: std::ffi::c_uint,
    pub Height: std::ffi::c_uint,
    pub PixelsPerScanLine: std::ffi::c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PSF1_HEADER {
    // PSF v1 header struct
    pub magic: [std::ffi::c_uchar; 2],
    pub mode: std::ffi::c_uchar,
    pub charsize: std::ffi::c_uchar,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PSF1_FONT {
    // PSF v1 font struct
    pub psf1_Header: *mut PSF1_HEADER,
    pub glyphBuffer: *mut std::ffi::c_void,
}

pub struct Point {
    pub X: u32,
    pub Y: u32,
}

pub static mut CursorPosition : Point = Point { X: 0u32, Y: 0u32 };
static mut framebuffer: *mut Framebuffer = std::ptr::null_mut(); // framebuffer need not be mutable after BasicRenderer fn Initializes it
static mut psf1_font: *mut PSF1_FONT = std::ptr::null_mut(); // psf1_font need not be mutable after BasicRenderer fn Initializes it
static mut colour : u32 = 0xFFFFFFu32;


pub fn BasicRenderer(FRAMEbuffer: *mut Framebuffer , PSF1_font: &mut PSF1_FONT) {
    unsafe {
         framebuffer = FRAMEbuffer;
         psf1_font = PSF1_font;
    }
}

// BasicRenderer
pub fn PutPix(x: u32, y: u32, color: u32) {
    // PutPix Function pass framebuffer colour and screen cords
    unsafe {
        ptr::write_volatile(
            (*framebuffer)
                .BaseAddress
                .add(((x * 4) + (y * (*framebuffer).PixelsPerScanLine * 4)) as usize)
                as *mut std::ffi::c_uint,
            color,
        );
    }
}

pub fn Clear(clear_color: u32) {
    unsafe {
        let mut x = 0;
        let mut y = 0;
        for _i in 0..(*framebuffer).BufferSize {
            // i in framebuffer size so in 1080 x 1920 i will be incremented 2073600 times
            let clear_color = std::mem::transmute(clear_color); // u32 color value 0x 00 <- R 00 <- G 00 <- B 00 <- alpha (not required so red color with no aplha is 0xFF0000u32)
            PutPix(x, y, clear_color); // calling PutPix passing framebuffer colour and screen cords
            if x < (*framebuffer).PixelsPerScanLine {
                // logic for passing on every pixel in screen
                x += 1;
            } else if y < (*framebuffer).Height {
                y += 1;
                x = 0;
            }
        }
        x = 0;
        y = 0;
    }
}

fn PutChar(
    chr: char,
    mut xOff: u32,
    mut yOff: u32,
) {
    // PutChar prints a character to the screen using the psf v1 font glyphbuffer
    unsafe {
        let mut fontPtr = (*psf1_font).glyphBuffer as *mut u8; // cast pointer to glyphbuffer
        for y in yOff..yOff + 16 {
            // hight of character is 16
            if y < (*framebuffer).Height {
                // check for cursor_position.Y (yOff) not going out of bounds and writing outside framebuffer into unspecified memory
                for x in xOff..xOff + 8 {
                    // width of character is 8
                    if x < (*framebuffer).Width {
                        // check for cursor_position.X (xOff) not going out of bounds and writing outside framebuffer into unspecified memory
                        let glyphIndex = (chr as u32) * (*(*psf1_font).psf1_Header).charsize as u32; // cast chr to u32 & dereference psf1_Header charsize and cast as u32
                        if *fontPtr.offset(glyphIndex.try_into().unwrap())
                            & (0b10000000 >> (x - xOff))
                            > 0
                        {
                            // bit shift andwise to iterate glyphbuffer for correct letter
                            if x < (*framebuffer).Width && y < (*framebuffer).Height {
                                // double check that cursor_position is not out of bounds
                                PutPix(x, y, colour);
                            }
                        }
                    }
                }
                fontPtr = fontPtr.offset(1); // add offset
            }
        }
    }
    xOff = 0; // clear values
    yOff = 0;
}

// should make it take str: impl Into<String>
pub fn Print(
    str: &str,
) {
    unsafe {
        // prints an &str by calling PutChar
        //let str = str.into();
        for c in str.chars() {
            // iterates characters of str pointer
            if CursorPosition.X < (*framebuffer).Width && CursorPosition.Y < (*framebuffer).Height {
                // checks for out of bounds
                PutChar(
                    c as char,
                    CursorPosition.X,
                    CursorPosition.Y,
                );
            }
            CursorPosition.X += 8;
            if CursorPosition.X + 8 > (*framebuffer).Width {
                // checks for X out of bounds and wraps over
                CursorPosition.X = 0;
                CursorPosition.Y += 16;
            }
        }
    }
}

pub fn Next() {
    unsafe {
         // my version of newline
        CursorPosition.X = 0; // reset X
        if CursorPosition.Y + 16 < (*framebuffer).Height {
            // checks for Y out of bounds and wraps over
            CursorPosition.Y += 16;
        } else {
            return; // scroll instead of wrapover when shell is implemented
        }
    }
}

pub fn Colour(color : u32) {
    unsafe {
        colour = color;
    }
}
