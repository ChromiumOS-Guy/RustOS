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
        let reminder: char = lazy_conversion(value % 10);
        value /= 10;
        int_to_string_output.push(reminder);
    }

    let reminder: char = lazy_conversion(value % 10);
    int_to_string_output.push(reminder);
    return ReverseString(int_to_string_output);
}

pub fn usize_to_str(mut value: usize) -> String {
    let mut int_to_string_output = String::with_capacity(128);
    while value / 10 > 0 {
        let reminder: char = lazy_conversion((value % 10) as u64);
        value /= 10;
        int_to_string_output.push(reminder);
    }

    let reminder: char = lazy_conversion((value % 10) as u64);
    int_to_string_output.push(reminder);
    return ReverseString(int_to_string_output);
}

pub fn u32_to_str(mut value: u32) -> String {
    let mut int_to_string_output = String::with_capacity(128);
    while value / 10 > 0 {
        let reminder: char = lazy_conversion((value % 10) as u64);
        value /= 10;
        int_to_string_output.push(reminder);
    }

    let reminder: char = lazy_conversion((value % 10) as u64);
    int_to_string_output.push(reminder);
    return ReverseString(int_to_string_output);
}

pub fn u16_to_str(mut value: u16) -> String {
    let mut int_to_string_output = String::with_capacity(128);
    while value / 10 > 0 {
        let reminder: char = lazy_conversion((value % 10) as u64);
        value /= 10;
        int_to_string_output.push(reminder);
    }

    let reminder: char = lazy_conversion((value % 10) as u64);
    int_to_string_output.push(reminder);
    return ReverseString(int_to_string_output);
}

pub fn u8_to_str(mut value: u8) -> String { 
    let mut int_to_string_output = String::with_capacity(128); 
    while value / 10 > 0 { 
        let reminder : char = lazy_conversion((value % 10) as u64);
        value /= 10;
        int_to_string_output.push(reminder);
    }

    let reminder : char = lazy_conversion((value % 10) as u64);
    int_to_string_output.push(reminder);
    return ReverseString(int_to_string_output);
}

pub fn u64hex_to_str(mut value: u64) -> String {
    let mut int_to_string_output = String::with_capacity(128);
    while value / 16 > 0 {
        let reminder: char = lazy_conversion(value % 16);
        value /= 16;
        int_to_string_output.push(reminder);
    }

    let reminder: char = lazy_conversion(value % 16);
    int_to_string_output.push(reminder);
    return ReverseString(int_to_string_output);
}

fn lazy_conversion(value: u64) -> char {
    if value == 0 {
        return '0';
    } else if value == 1 {
        return '1';
    } else if value == 2 {
        return '2';
    } else if value == 3 {
        return '3';
    } else if value == 4 {
        return '4';
    } else if value == 5 {
        return '5';
    } else if value == 6 {
        return '6';
    } else if value == 7 {
        return '7';
    } else if value == 8 {
        return '8';
    } else if value == 9 {
        return '9';
    } else if value == 10 {
        return 'A';
    } else if value == 11 {
        return 'B';
    } else if value == 12 {
        return 'C';
    } else if value == 13 {
        return 'D';
    } else if value == 14 {
        return 'E';
    } else if value == 15 {
        return 'F';
    }
    return '0'; //defualts to a 0
}

fn ReverseString(string: String) -> String { // weird stuff but this apparently has no memory leaks 
    let mut int_to_string = Vec::<char>::with_capacity(128);
    for c in string.chars().rev() {
        int_to_string.push(c)
    }
    let mut int_to_string_output2 = String::with_capacity(128);
    for c in int_to_string {
        int_to_string_output2.push(c);
    }
    return int_to_string_output2;
}
