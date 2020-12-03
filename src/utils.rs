use std::fs;
 
pub fn read_day(day: u8)-> Result<Box<str>, std::io::Error> {
    Ok(fs::read_to_string(format!("{}.txt", day))?.into_boxed_str())
}

pub fn read_test(day: u8)-> Result<Box<str>, std::io::Error> {
    Ok(fs::read_to_string(format!("test{}.txt", day))?.into_boxed_str())
}