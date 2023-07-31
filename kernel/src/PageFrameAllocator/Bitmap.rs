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
    pub buffer: Vec<u8>,
}

impl Bitmap { // just remember to set all places in bitmap 
    pub fn get(&self, index: usize) -> bool {
        if index >= (self.size * 8) {
            return false;
        }
        let byte_index = index / 8;
        let bit_index = index % 8;
        let bit_mask = 1 << bit_index;
        return (self.buffer[byte_index] & bit_mask) != 0;
    }

    pub fn set(&mut self, index: usize, value: bool) -> bool {
        if index >= self.size * 8 {
            return false;
        }
        let byte_index = index / 8;
        let bit_index = index % 8;
        let bit_mask = 1 << bit_index;
        let mut byte = self.buffer[byte_index];
        if value {
            byte |= bit_mask;
        } else {
            byte &= !bit_mask;
        }
        self.buffer[byte_index] = byte;
        return true;
    }
}