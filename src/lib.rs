use std::io::{self, Read, Write};

use termios::{ECHO, ICANON, TCSANOW, Termios, tcsetattr};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid data pointer")]
    DataPointerOutOfBounds,
    #[error("invalid memory access")]
    InvalidMemoryAccess,
    #[error("could not convert {0} into a valid char")]
    InvalidCharacter(u8),
    #[error("'{0}' is not a valid bracket")]
    InvalidBracket(u8),
    #[error("no matching bracket found for symbol at <{0}>")]
    NoMatchingBracket(usize),
    #[error("end of program has been reached")]
    EndOfProgram,
    #[error(transparent)]
    Io(#[from] ::std::io::Error),
}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

#[derive(Debug)]
pub struct AbstractMachine<'a> {
    // Data pointer, indicating the current cell being pointed at.
    dp: usize,
    // The one-dimensional tape of memory cells that the Brainfuck program operates.
    mem: Vec<u8>,
    // Instruction pointer, which points to the next command to be executed.
    ip: usize,
    // The actual Brainfuck source code that we're running.
    program: &'a [u8],
    // FIXME
    stack: Vec<usize>,
}

impl<'a> AbstractMachine<'a> {
    pub const DEFAULT_NUM_CELLS: usize = 30_000;

    /// Creates a new Brainfuck abstract machine to run the given program.
    pub fn new(program: &'a [u8]) -> Self {
        Self {
            dp: 0,
            mem: vec![0_u8; Self::DEFAULT_NUM_CELLS],
            ip: 0,
            program,
            stack: Vec::new(),
        }
    }

    /// Given an abstract machine, it initializes its memory with [`num_cells`] set to zero.
    pub fn with_num_cells(mut self, num_cells: usize) -> Self {
        self.mem = vec![0_u8; num_cells];
        self
    }

    #[cfg(test)]
    fn with_mem(mut self, mem: Vec<u8>) -> Self {
        self.mem = mem;
        self
    }

    #[cfg(test)]
    fn with_stack(mut self, stack: Vec<usize>) -> Self {
        self.stack = stack;
        self
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            if let Err(e) = self.step() {
                match e {
                    Error::EndOfProgram => {
                        break;
                    }
                    _ => return Err(e),
                }
            }
        }

        Ok(())
    }

    /// Executes the next command indexed by the instruction pointer.
    pub fn step(&mut self) -> Result<()> {
        let Some(command) = self.program.get(self.ip) else {
            return Err(Error::EndOfProgram);
        };

        let ip = match command {
            b'>' => self.execute_shr()?,
            b'<' => self.execute_shl()?,
            b'+' => self.execute_inc()?,
            b'-' => self.execute_dec()?,
            b'.' => self.execute_out()?,
            b',' => self.execute_in()?,
            b'[' => self.execute_openbrk()?,
            b']' => self.execute_closebrk()?,
            _ => InstructionPointer::Next,
        };

        self.ip = match ip {
            InstructionPointer::Next => self.ip + 1,
            InstructionPointer::Jump(addr) => addr,
        };

        Ok(())
    }

    #[inline]
    fn read_byte(&self) -> Result<u8> {
        Ok(*self.mem.get(self.dp).ok_or(Error::InvalidMemoryAccess)?)
    }

    #[inline]
    fn write_byte(&mut self, value: u8) -> Result<()> {
        let byte = self
            .mem
            .get_mut(self.dp)
            .ok_or(Error::InvalidMemoryAccess)?;
        *byte = value;

        Ok(())
    }

    // increment the data pointer by one (move right)
    fn execute_shr(&mut self) -> Result<InstructionPointer> {
        let (value, overflow) = self.dp.overflowing_add(1);
        if overflow {
            return Err(Error::DataPointerOutOfBounds);
        }
        self.dp = value;

        Ok(InstructionPointer::Next)
    }

    // decrement the data pointer by one (move left)
    fn execute_shl(&mut self) -> Result<InstructionPointer> {
        let (value, overflow) = self.dp.overflowing_sub(1);
        if overflow {
            return Err(Error::DataPointerOutOfBounds);
        }
        self.dp = value;

        Ok(InstructionPointer::Next)
    }

    // increment the byte at the data pointer by one modulo 256.
    fn execute_inc(&mut self) -> Result<InstructionPointer> {
        let byte = self.read_byte()?;
        self.write_byte(byte.wrapping_add(1))?;

        Ok(InstructionPointer::Next)
    }

    // decrement the byte at the data pointer by one modulo 256.
    fn execute_dec(&mut self) -> Result<InstructionPointer> {
        let byte = self.read_byte()?;
        self.write_byte(byte.wrapping_sub(1))?;

        Ok(InstructionPointer::Next)
    }

    // accept one byte of input, storing its value in the byte at the data pointer
    fn execute_in(&mut self) -> Result<InstructionPointer> {
        let c = getchar()?;
        self.write_byte(c)?;

        Ok(InstructionPointer::Next)
    }

    // output the byte at the data pointer
    fn execute_out(&mut self) -> Result<InstructionPointer> {
        let byte = self.read_byte()?;
        let c = char::from_u32(byte.into()).ok_or(Error::InvalidCharacter(byte))?;
        let mut lock = io::stdout().lock();
        write!(lock, "{}", c)?;

        Ok(InstructionPointer::Next)
    }

    // if the byte at the data pointer is zero, instead of moving the instruction pointer
    // forward to the next command, jump it forward to the command after the matching ']'
    // command
    fn execute_openbrk(&mut self) -> Result<InstructionPointer> {
        if self.read_byte()? == 0 {
            let matching_pos = find_matching(self.ip, self.program)?;
            self.ip = matching_pos + 1;

            Ok(InstructionPointer::Jump(matching_pos + 1))
        } else {
            // we only insert the location of the `[` if it isn't already on the top of the
            // stack
            let must_insert = self.stack.last().map(|&x| x != self.ip).unwrap_or(true);
            if must_insert {
                self.stack.push(self.ip);
            }

            Ok(InstructionPointer::Next)
        }
    }

    // if the byte at the data pointer is nonzero, then instead of moving the instruction
    // pointer forward to the next command, jump it back to the command after the matching
    // '['
    fn execute_closebrk(&mut self) -> Result<InstructionPointer> {
        if self.read_byte()? == 0 {
            self.stack.pop();
            Ok(InstructionPointer::Next)
        } else {
            let pos = *self.stack.last().ok_or(Error::NoMatchingBracket(self.ip))?;
            Ok(InstructionPointer::Jump(pos + 1))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum InstructionPointer {
    Next,
    Jump(usize),
}

fn find_matching(pos: usize, code: &[u8]) -> Result<usize> {
    let mut curr = pos + 1;
    let mut skipped = 0_u32;

    while curr < code.len() {
        if code[curr] == b']' {
            if skipped == 0 {
                return Ok(curr);
            } else {
                skipped -= 1;
            }
        } else if code[curr] == b'[' {
            skipped += 1;
        }

        curr += 1;
    }

    Err(Error::NoMatchingBracket(pos))
}

fn getchar() -> Result<u8> {
    let fd = 0; // stdin
    // Fetch the current termios struct, so that we can restore once we're done.
    let curr_termios = Termios::from_fd(fd)?;
    // Copy the current termios, set the flags we're interested in, and then apply
    let mut new_termios = curr_termios;
    // We're doing two things here:
    // * disabling canonical mode
    // * disabling echoing input characters
    //
    // Canonical mode is set by default. In canonical mode, the input is made available line by
    // line, when the line delimiter is inserted. Except for EOL, the line delimiter is included in
    // the buffer returned by read. We don't want that. In noncanonical mode, the input is made
    // available.
    new_termios.c_lflag &= !(ICANON | ECHO);
    // Set the parameters associated with the terminal from the new_termios struct. The flag TCSANOW
    // means that changes are effective immediately.
    tcsetattr(fd, TCSANOW, &new_termios)?;
    // We want to read exactly one byte from stdin
    let mut reader = io::stdin();
    let mut buffer = [0_u8; 1];
    io::stdout().lock().flush()?;
    reader.read_exact(&mut buffer)?;
    // Restore stdin with original values of termios struct
    tcsetattr(fd, TCSANOW, &curr_termios)?;

    Ok(buffer[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment_and_decrement_data_pointer() {
        let program = [b'>', b'<'];
        let mut machine = AbstractMachine::new(&program);
        machine.step().expect("valid operation >");
        assert_eq!(1, machine.dp);
        machine.step().expect("valid operation <");
        assert_eq!(0, machine.dp);
    }

    #[test]
    fn increment_and_decrement_byte_at_data_pointer() {
        let program = [b'+', b'-'];
        let mut machine = AbstractMachine::new(&program);
        machine.step().expect("valid operation +");
        assert_eq!(1, machine.mem[0]);
        machine.step().expect("valid operation -");
        assert_eq!(0, machine.mem[0]);
    }

    #[test]
    fn find_matching_success() {
        let program = [b'[', b']'];
        assert_eq!(1, find_matching(0, &program).expect("should find matching"));
        let program = [b'>', b']', b'[', b'.', b']'];
        assert_eq!(4, find_matching(2, &program).expect("should find matching"));
        let program = [b'[', b'[', b'[', b']', b']', b']'];
        for (matching, pos) in [(5, 0), (4, 1), (3, 2)] {
            assert_eq!(
                matching,
                find_matching(pos, &program).expect("should find matching")
            );
        }
    }

    #[test]
    fn find_matching_failure() {
        let program = [b'[', b']', b'[', b'>', b'[', b']'];
        assert_eq!(5, find_matching(4, &program).expect("should find matching"));
        assert!(find_matching(2, &program).is_err());
    }

    #[test]
    fn insert_open_bracket_on_stack() {
        let program = [b'[', b'+', b']', b'>'];
        let mut machine = AbstractMachine::new(&program).with_mem(vec![1, 2, 3]);
        machine.step().expect("valid operation");
        assert_eq!(machine.stack.last(), Some(&0));
    }

    #[test]
    fn skip_insert_existing_open_bracket_on_stack() {
        let program = [b'[', b'+', b']', b'>'];
        let mut machine = AbstractMachine::new(&program)
            .with_mem(vec![1, 2, 3])
            .with_stack(vec![0]);
        machine.step().expect("valid operation");
        assert_eq!(machine.stack.last(), Some(&0));
        assert_eq!(1, machine.stack.len());
    }

    #[test]
    fn jump_to_instruction_after_matching_open_bracket() {
        let program = [b'[', b'+', b']', b'>'];
        let mut machine = AbstractMachine::new(&program)
            .with_mem(vec![1, 2, 3])
            .with_stack(vec![0]);
        machine.ip = 2; // ip points to ']'
        machine.step().expect("valid operation");
        // instruction pointer points at the instruction after '['
        assert_eq!(1, machine.ip);
    }
}
