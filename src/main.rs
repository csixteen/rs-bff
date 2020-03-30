extern crate clap;
extern crate termios;

use std::char;
use std::collections::VecDeque;
use std::fs;
use std::io::{self, Read, Write};

use clap::{Arg, App};

use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};


// --------------------------------------------------
// Helpers

// Reads a single char from stdin without the need for the user to
// hit "Enter".
// https://stackoverflow.com/questions/26321592/how-can-i-read-one-character-from-stdin-without-having-to-hit-enter
fn getch() -> u8 {
    let stdin = 0;
    let termios = Termios::from_fd(stdin).unwrap();
    let mut new_termios = termios.clone();

    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();
    let stdout = io::stdout();
    let mut reader = io::stdin();
    let mut buffer = [0;1];  // read exactly one byte
    stdout.lock().flush().unwrap();
    reader.read_exact(&mut buffer).unwrap();
    tcsetattr(stdin, TCSANOW, & termios).unwrap();

    buffer[0]
}


// --------------------------------------------------
// `Program` struct and implementation

struct Program {
    code: Vec<char>,         // Brainfuck code
    ip: usize,               // Instruction pointer
    cursor: usize,           // Memory cursor
    stack: VecDeque<usize>,  // Program stack
    cells: Vec<u8>,          // 8-bit cells
}

impl Program {
    fn new(code: Vec<char>, n: usize) -> Program {
        Program {
            code: code,
            ip: 0,
            cursor: 0,
            stack: VecDeque::new(),
            cells: vec![0; n],
        }
    }

    fn execute_instruction(&mut self) {
        let mut offset: i32 = 1;

        match self.code[self.ip] {
            '<' => self.cursor = self.cursor.wrapping_sub(1),
            '>' => self.cursor = self.cursor.wrapping_add(1),
            '+' => self.cells[self.cursor] = self.cells[self.cursor].wrapping_add(1),
            '-' => self.cells[self.cursor] = self.cells[self.cursor].wrapping_sub(1),
            '.' => {
                let c = char::from_u32(self.cells[self.cursor].into()).unwrap();
                print!("{}", c);
            }
            ',' => self.cells[self.cursor] = getch(),
            '[' => {
                if self.cells[self.cursor] > 0 {
                    self.stack.push_front(self.ip);
                } else {
                    let mut i = self.ip + 1;
                    let mut skipped = 0;

                    while i < self.code.len() {
                        match self.code[i] {
                            '[' => skipped += 1,
                            ']' => if skipped == 0 {
                                i += 1;
                                break;
                            } else {
                                skipped -= 1;
                            }
                            _ => (),
                        }

                        i += 1;
                    }

                    offset = i as i32 - self.ip as i32;
                }
            }
            ']' => {
                if self.cells[self.cursor] == 0 {
                    self.stack.pop_front();
                } else {
                    offset = (self.stack[0] as i32) - (self.ip as i32) + 1;
                }
            }
            _ => (),
        }

        self.ip = self.ip.wrapping_add(offset as usize)
    }

    fn execute(&mut self) {
        while self.ip < self.code.len() {
            self.execute_instruction();
        }
    }
}


// ------------------------------------------------------


// Reads the contents of a source file and returns
// a Vector with the chars that only represent valid
// Brainfuck operators: <>+-,.[]
fn load_code(file_name: &str) -> Vec<char> {
    let contents = fs::read_to_string(file_name)
        .expect("Couldn't read from file");

    contents
        .chars()
        .filter(|c| {
            match c {
                '<' | '>' | '+' | '-' | ',' | '.' | '[' | ']' => true,
                _ => false,
            }
        })
        .collect::<Vec<char>>()
}


// -------------------------------------------------------


// Entry point
fn main() {
    let matches = App::new("Brainfuck interpreter in Rust")
                        .version("0.1.0")
                        .author("Pedro Rodrigues <csixteen@protonmail.com>")
                        .arg(Arg::with_name("file_name")
                             .value_name("FILE")
                             .help("File with Brainfuck source code.")
                             .takes_value(true)
                             .required(true))
                        .arg(Arg::with_name("num_cells")
                             .short("n")
                             .long("num-cells")
                             .value_name("N")
                             .help("Number of cells (default: 30,000)")
                             .takes_value(true))
                        .get_matches();

    let file_name = matches.value_of("file_name").unwrap();
    let num_cells: usize = matches.value_of("num_cells")
                                    .unwrap_or("30000")
                                    .parse()
                                    .unwrap();

    Program::new(
        load_code(file_name),
        num_cells,
    ).execute();
}
