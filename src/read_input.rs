use std::fs;

#[inline(always)]
pub fn read_input(day: u8) -> String {
    fs::read_to_string(&format!("input/day{}", day))
        .expect(&format!("Failed to read file day {}", day))
}

pub fn read_input_bytes(day: u8) -> Vec<u8> {
    fs::read(&format!("input/day{}", day)).unwrap()
}
