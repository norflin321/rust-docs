use std::fs::File;
use std::io::{self, Read};

pub fn calc_length(str: &mut String) -> usize {
	str.push_str("?");
	str.len()
}

pub fn get_slice(str: &String) -> &str {
	&str[0..str.len()]
}

// when "?" placed after Result, which is Ok, it will return its value,
// if its Err than that Err will be return from the whole function
pub fn read_from_file() -> Result<String, io::Error> {
	let mut buf = String::new();
	File::open("readme")?.read_to_string(&mut buf)?;
	Ok(buf)
}