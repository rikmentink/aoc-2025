use std::fs;
use std::io;

pub fn read_input(day: u8) -> io::Result<String> {
    let path = format!("inputs/day{:02}.txt", day);
    fs::read_to_string(&path)
}

