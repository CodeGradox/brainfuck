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

	let buf: Vec<char> = match env::args().nth(1) {
		Some(ref s) => {
			read_file(s)
				.expect("Error encoutered while opening file")
				.chars()
				.collect()
			},
		None => {
			println!("No file given!");
			return;
		},
	};

	while buf_pos != buf.len() {
		match buf[buf_pos] {
			'+' => tape[head] = tape[head].wrapping_add(1),
			'-' => tape[head] = tape[head].wrapping_sub(1),
			'>' => head += 1,
			'<' => head -= 1,
			'.' => print!("{}", tape[head] as char),
			',' => {
				match stdin.lock().chars().next() {
					Some(c) => tape[head] = c.unwrap_or('\0') as u8,
					None => println!("Could not read input."),
				}
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
