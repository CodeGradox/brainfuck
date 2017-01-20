use std::io::Read;
use std::io::BufRead;
use std::fs::File;
use std::env;

fn main() {
    let mut tape = [0u8; 1000];
    let mut head = 0;
    let mut buf_pos = 0;
    let stdin = std::io::stdin();

    let file = env::args().nth(1).expect("No argument given.");
    let mut str_buf = String::new();
    File::open(&file)
        .and_then(|mut f| f.read_to_string(&mut str_buf))
        .expect(&format!("Could not read {}", file));
    let buf: Vec<char> = str_buf.chars().collect();

    while buf_pos != buf.len() {
        match buf[buf_pos] {
            '+' => tape[head] = tape[head].wrapping_add(1),
            '-' => tape[head] = tape[head].wrapping_sub(1),
            '>' => head += 1,
            '<' => head -= 1,
            '.' => print!("{}", tape[head] as char),
            ',' => {
                let mut line = String::new();
                stdin.lock().read_line(&mut line).expect("Error Reading input.");
                line.chars().next().map(|x| tape[head] = x as u8).expect("Empty input.");
            }
            '[' => {
                if tape[head] == 0 {
                    let mut level = 0;
                    while buf_pos != buf.len() {
                        buf_pos += 1;
                        match buf[buf_pos] {
                            '[' => level += 1,
                            ']' if level == 0 => break,
                            ']' => level -= 1,
                            _ => {}
                        }
                    }
                }
            }
            ']' => {
                if tape[head] != 0 {
                    let mut level = 0;
                    while buf_pos > 0 {
                        buf_pos -= 1;
                        match buf[buf_pos] {
                            ']' => level += 1,
                            '[' if level == 0 => break,
                            '[' => level -= 1,
                            _ => {}
                        }
                    }
                }
            }
            _ => {} 
        }
        buf_pos += 1;
    }
}
