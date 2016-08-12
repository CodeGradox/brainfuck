#![feature(io)]

use std::io::{Read, Error as IOError};
use std::fs::File;
use std::env;

fn read_file(file: &str) -> Result<String, IOError> {
	let mut file = try!(File::open(file));
	let mut buf = String::new();
	try!(file.read_to_string(&mut buf));
	Ok(buf)
}

fn main() {
	let mut tape = [0u8; 1000];
	let mut head: usize = 0;
	let mut buf_pos: usize = 0;
	let stdin = std::io::stdin();

	let buf: Vec<char> = env::args()
		.nth(1)
		.map(|s| read_file(&s)
			.expect(&format!("\"{}\" not found", s))
			.chars()
			.collect())
		.expect("No argument given");

	while buf_pos != buf.len() {
		match buf[buf_pos] {
			'+' => tape[head] = tape[head].wrapping_add(1),
			'-' => tape[head] = tape[head].wrapping_sub(1),
			'>' => head += 1,
			'<' => head -= 1,
			'.' => print!("{}", tape[head] as char),
			',' => {
				stdin.lock()
					.chars()
					.next()
					.map(|x| tape[head] = x.expect("Error reading input") as u8);
			},
			'[' => {
				if tape[head] == 0 {
					let mut level = 0;
					while buf_pos != buf.len() {
						buf_pos += 1;
						match buf[buf_pos] {
							'[' => level += 1,
							']' if level == 0 => break,
							']' => level -= 1,
							_ => {},
						}
					}
				}
			},
			']' => {
				if tape[head] != 0 {
					let mut level = 0;
					while buf_pos > 0 {
						buf_pos -= 1;
						match buf[buf_pos] {
							']' => level += 1,
							'[' if level == 0 => break,
							'[' => level -= 1,
							_ => {},
						}
					}
				}
			},
			_ => { /* Ignore all other characters */ },
		}
		buf_pos += 1;
	}
}
