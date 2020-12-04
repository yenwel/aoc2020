use std::fs;
 
pub fn read_day(day: u8)-> Result<Box<str>, std::io::Error> {
    Ok(fs::read_to_string(format!("{}.txt", day))?.into_boxed_str())
}

pub fn read_test(day: u8, part: Option<&str>)-> Result<Box<str>, std::io::Error> {
    let part = match part {
        Some(part) => format!("-{}", part),
        None => "".to_owned()
    };
    Ok(fs::read_to_string(format!("test{}{}.txt", day, part))?.into_boxed_str())
}
