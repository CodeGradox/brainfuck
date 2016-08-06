// #![feature(io)]

use std::io::{Read, Error as IOError};
use std::fs::File;

fn read_file() -> Result<String, IOError> {
	let mut file = try!(File::open("tests/hello_world.txt"));
	let mut buf = String::new();
	try!(file.read_to_string(&mut buf));
	Ok(buf)
}

fn main() {
	let mut tape = [0u8; 500];
	let mut head: usize = 0;
	let mut buf_pos: usize = 0;
	let buf: Vec<char> = read_file()
		.expect("Could not open file")
		.chars()
		.collect();

	while buf_pos != buf.len() {
		match buf[buf_pos] {
			'+' => tape[head] = tape[head].wrapping_add(1),
			'-' => tape[head] = tape[head].wrapping_sub(1),
			'>' => head += 1,
			'<' => head -= 1,
			'.' => print!("{}", tape[head] as char),
			// ',' => {
			// 	let input = std::io::stdin();
			// 	match input.lock().chars().nth(0) {
			// 		Some(c) => { tape[head] = c.unwrap() as u8 },
			// 		None => { println!("Could not read input."); },
			// 	}
			// },
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
		// println!("head: {} tape: {:?}", head, tape);
	}

	// println!("{:?}", tape);
}
