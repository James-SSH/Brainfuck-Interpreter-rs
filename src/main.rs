use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, Read};

const ALLOWED_KEYCHARS: &str = "<>+-[],.";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut instruction_pointer: usize = 0;
    let mut tape_pointer: usize = 0;
    let mut tape: [u8; 30_000] = [0; 30000];

    let mut bf_string: String = String::new();
    match File::open(&args[1]) {
        Ok(mut fh) => match fh.read_to_string(&mut bf_string) {
            Ok(bytes_read) => {
                eprintln!("{}B read", bytes_read)
            }
            Err(err) => {
                panic!("{:#?}", err)
            }
        },
        Err(err) => {
            panic!("{:#?}", err)
        }
    };

    match validate(bf_string.trim_end()) {
        Ok(_) => {}
        Err(err) => {
            panic!("Brainfuck validation error: {}", err)
        }
    }

    let mut jump_points: Vec<usize> = Vec::new();
    let instructions = bf_string.as_bytes();
    while instruction_pointer != instructions.len() {
        match instructions[instruction_pointer] as char {
            '<' => tape_pointer -= 1,
            '>' => tape_pointer += 1,
            '+' => tape[tape_pointer] += 1,
            '-' => tape[tape_pointer] -= 1,
            '[' => {
                jump_points.push(instruction_pointer);
            }
            ']' => {
                if tape[tape_pointer] > 0 {
                    instruction_pointer = *jump_points.last().unwrap();
                    continue;
                } else {
                    jump_points.pop();
                }
            }
            ',' => {
                tape[tape_pointer] = std::io::stdin()
                    .lock()
                    .lines()
                    .next()
                    .unwrap_or(Ok(String::from("")))
                    .unwrap()
                    .as_bytes()[0]
            }
            '.' => {
                print!("{}", tape[tape_pointer] as char)
            }
            _ => {}
        }
        instruction_pointer += 1;
    }
}

fn validate(bf_str: &str) -> Result<(), ValidationError> {
    if !bf_str.is_ascii() {
        return Err(ValidationError {
            message: String::from("String does not contain entirely ASCII characters"),
        });
    }

    if !bf_str.chars().all(|kc| ALLOWED_KEYCHARS.contains(kc)) {
        return Err(ValidationError {
            message: format!(
                "Invalid Char: {}, at idx: {}",
                bf_str.as_bytes()[bf_str.find(|kc| !ALLOWED_KEYCHARS.contains(kc)).unwrap()] as char,
                bf_str.find(|kc| !ALLOWED_KEYCHARS.contains(kc)).unwrap()
            ),
        });
    }

    {
        let mut bracket_pair: Vec<(char, usize)> = Vec::new();
        for kc in bf_str.chars().enumerate() {
            match kc.1 {
                '[' => bracket_pair.push(('[', kc.0)),
                ']' => {
                    if bracket_pair.pop().is_none() {
                        return Err(ValidationError {
                            message: format!("Unexpected closing bracket at: {}", kc.0),
                        });
                    }
                }
                _ => {}
            }
        }
        if !bracket_pair.is_empty() {
            return Err(ValidationError {
                message: format!(
                    "Unclosed bracket{plural} at index{plural}: {idxes}",
                    plural = if bracket_pair.len() > 1 { "s" } else { "" },
                    idxes = bracket_pair
                        .iter()
                        .map(|bp| bp.1.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
            });
        }
    }

    Ok(())
}

struct ValidationError {
    message: String,
}

impl Debug for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "BrainFuck Validation Error: {}",
            &self.message
        ))
    }
}

impl Error for ValidationError {}
