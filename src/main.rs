extern crate termios;

use std::char;
use std::collections::VecDeque;
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::process;

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
    memory: Vec<u8>,         // Memory
}

impl Program {
    fn new(code: Vec<char>) -> Program {
        Program {
            code: code,
            ip: 0,
            cursor: 0,
            stack: VecDeque::new(),
            memory: vec![0; 30000],
        }
    }

    fn execute_instruction(&mut self) {
        let mut offset: i32 = 1;

        match self.code[self.ip] {
            '<' => self.cursor -= 1,
            '>' => self.cursor += 1,
            '+' => self.memory[self.cursor] += 1,
            '-' => if self.memory[self.cursor] > 0 {
                self.memory[self.cursor] -= 1;
            }
            '.' => {
                let c = char::from_u32(self.memory[self.cursor].into()).unwrap();
                print!("{}", c);
            }
            ',' => self.memory[self.cursor] = getch(),
            '[' => {
                if self.memory[self.cursor] > 0 {
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
                if self.memory[self.cursor] == 0 {
                    self.stack.pop_front();
                } else {
                    offset = (self.stack[0] as i32) - (self.ip as i32) + 1;
                }
            }
            _ => (),
        }

        // I'm still a bit unsure how to properly solve this case.
        // If I just leave as `self.ip += offset as usize` then
        // I get a runtime error saying that I'm attempting to subtract
        // with overflow.
        self.ip = if offset < 0 {
            self.ip - offset.abs() as usize
        } else {
            self.ip + offset as usize
        }
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
fn load_code(file_name: String) -> Vec<char> {
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
    let mut args = env::args();

    args.next();

    let file_name = match args.next() {
        Some(arg) => arg,
        None => {
            eprintln!("Missing source file. Usage: <prog> <filename.bf>");

            process::exit(1);
        }
    };

    Program::new(load_code(file_name)).execute();
}
