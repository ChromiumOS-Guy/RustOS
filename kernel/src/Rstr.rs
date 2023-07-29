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

pub fn u64_to_str(mut value: u64) -> String {
    let mut int_to_string_output = String::with_capacity(128);
    while value / 10 > 0 {
        let reminder: u8 = lazy_conversion(value % 10);
        value /= 10;
        int_to_string_output.push((reminder + '0' as u8) as char);
    }

    let reminder: u8 = lazy_conversion(value % 10);
    int_to_string_output.push((reminder + '0' as u8) as char);
    return int_to_string_output;
}

pub fn usize_to_str(mut value: usize) -> String {
    let mut int_to_string_output = String::with_capacity(128);
    while value / 10 > 0 {
        let reminder: u8 = lazy_conversion((value % 10) as u64);
        value /= 10;
        int_to_string_output.push((reminder + '0' as u8) as char);
    }

    let reminder: u8 = lazy_conversion((value % 10) as u64);
    int_to_string_output.push((reminder + '0' as u8) as char);
    return int_to_string_output;
}

pub fn u32_to_str(mut value: u32) -> String {
    let mut int_to_string_output = String::with_capacity(128);
    while value / 10 > 0 {
        let reminder: u8 = lazy_conversion((value % 10) as u64);
        value /= 10;
        int_to_string_output.push((reminder + '0' as u8) as char);
    }

    let reminder: u8 = lazy_conversion((value % 10) as u64);
    int_to_string_output.push((reminder + '0' as u8) as char);
    return int_to_string_output;
}

pub fn u16_to_str(mut value: u16) -> String {
    let mut int_to_string_output = String::with_capacity(128);
    while value / 10 > 0 {
        let reminder: u8 = lazy_conversion((value % 10) as u64);
        value /= 10;
        int_to_string_output.push((reminder + '0' as u8) as char);
    }

    let reminder: u8 = lazy_conversion((value % 10) as u64);
    int_to_string_output.push((reminder + '0' as u8) as char);
    return int_to_string_output;
}

pub fn u8_to_str(mut value: u8) -> String { // it all basically the same so i make notes only on this one
    let mut int_to_string_output = String::with_capacity(128); // i make an int_to_string_output with capacity of 128 which in my opinion is more than enough
    while value / 10 > 0 { // so basically if i have 100 and i modulo by 10 i get 0 if i do it again i get 0
        let reminder = value % 10;
        value /= 10;
        int_to_string_output.push((reminder + '0' as u8) as char);
    }

    let reminder = value % 10; // and if i do it again i get 1 assemble reverse and you get 100
    int_to_string_output.push((reminder + '0' as u8) as char);
    return int_to_string_output; // int_to_string_output.chars().rev().collect() makes a memory leak for now numbers are reversed deal with it.
}

fn lazy_conversion(value: u64) -> u8 {
    // temporary fix until i care enough (which is never)
    if value == 0 {
        return 0u8;
    } else if value == 1 {
        return 1u8;
    } else if value == 2 {
        return 2u8;
    } else if value == 3 {
        return 3u8;
    } else if value == 4 {
        return 4u8;
    } else if value == 5 {
        return 5u8;
    } else if value == 6 {
        return 6u8;
    } else if value == 7 {
        return 7u8;
    } else if value == 8 {
        return 8u8;
    } else if value == 9 {
        return 9u8;
    }
    return 0u8; //defualts to a 0
}
