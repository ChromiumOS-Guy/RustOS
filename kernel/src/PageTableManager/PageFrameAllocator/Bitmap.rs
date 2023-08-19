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

#[derive(Debug)]
pub struct Bitmap {
    pub size: usize,
    pub buffer: *mut u8,
}

impl Bitmap {
    pub const fn new_placeholder() -> Bitmap {
        let size = 1;
        let buffer = std::ptr::null_mut();
        return Bitmap {
            size,
            buffer,
        };
    }

    pub fn new(size: usize, buffer: *mut std::ffi::c_void) -> Bitmap {
        let mut bitmap = Bitmap {
            size,
            buffer: unsafe {std::mem::transmute(buffer)},
        };
        //Set all places in the bitmap to zero.
        for i in 0..size {
            unsafe {std::ptr::write_volatile(bitmap.buffer.add(i), 0)};
        }
        return bitmap;
    }

    pub unsafe fn get(&self, index: usize) -> bool {
        // Get the value of the bit at the specified index.
        if index >= (self.size * 8) {
            return false;
        }
        return std::ptr::read_volatile(self.buffer.add(index)) != 0;
    }

    pub unsafe fn set(&mut self, index: usize, value: bool) -> bool {
        // Set the value of the bit at the specified index.
        if index >= (self.size * 8) {
            return false;
        }
        let byte = (value as u8) & 1;
        std::ptr::write_volatile(self.buffer.add(index), byte);
        return true;
    }
}


