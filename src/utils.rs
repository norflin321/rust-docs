use std::fmt::Debug;
use std::fs::File;
use std::io::{self, Read};

use crate::traits::Movable;

#[derive(Debug, Clone)]
pub struct Point<T> { pub x: T, pub y: T }
impl Point<f32> {
	// Point<T> where T is not of type f32 will not have this method defined,
	// rust will generate code at compile time for the concrete types the generic code is called.
	pub fn zero() -> Self {
		Self{ x: 0.0, y: 0.0 }
	}
}

pub fn calc_length(str: &mut String) -> usize {
	str.push_str("?");
	str.len()
}

pub fn get_slice(str: &String) -> &str {
	&str[0..str.len()]
}

/// when "?" placed after Result, which is Ok, it will return its value, if its Err than that Err will be return from the whole function
pub fn read_from_file() -> Result<String, io::Error> {
	let mut buf = String::new();
	File::open("readme")?.read_to_string(&mut buf)?;
	Ok(buf)
}

/// we can combine traits with generic types to constrain it to accept only those types that have a particular behavior
pub fn move_to_start<T: Movable + Debug>(movable: &mut T) {
	movable.move_to(0.0, 0.0);
	println!("movable moved to start");
}

/// when we pass concrete references to this function, the generic lifetime ('a) will get the concrete lifetime that is equal to the smaller
/// of the lifetimes of x and y. Because we’ve annotated the returned reference with the same lifetime parameter ('a),
/// the returned reference will also be valid for the length of the smaller of the lifetimes of x and y.
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() > y.len() {
		return x
	}
	y
}