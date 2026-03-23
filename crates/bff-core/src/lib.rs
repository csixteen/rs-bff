mod error;
mod ext;

use std::{
    io,
    sync::{Arc, RwLock},
};

pub use self::{error::*, ext::*};

pub type Reader = Arc<RwLock<dyn ReadOne>>;
pub type Writer = Arc<RwLock<dyn io::Write>>;

pub struct AbstractMachine {
    // Data pointer, indicating the current cell being pointed at.
    dp: usize,
    // The one-dimensional tape of memory cells that the Brainfuck program operates.
    mem: Vec<u8>,
    // Instruction pointer, which points to the next command to be executed.
    ip: usize,
    // The actual Brainfuck source code that we're running.
    program: Arc<[u8]>,
    // Stack used to sture the program offsets with the location of opening square brackets
    stack: Vec<usize>,
    reader: Reader,
    // A writer where the output command will write onto.
    writer: Writer,
}

impl AbstractMachine {
    pub const DEFAULT_NUM_CELLS: usize = 30_000;

    /// Creates a new Brainfuck abstract machine to run the given program, a reader and a writer.
    pub fn new(program: Arc<[u8]>, reader: Reader, writer: Writer) -> Self {
        Self {
            dp: 0,
            mem: vec![0_u8; Self::DEFAULT_NUM_CELLS],
            ip: 0,
            program,
            stack: Vec::new(),
            reader,
            writer,
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
    #[inline]
    fn execute_shr(&mut self) -> Result<InstructionPointer> {
        let (value, overflow) = self.dp.overflowing_add(1);
        if overflow {
            return Err(Error::DataPointerOutOfBounds);
        }
        self.dp = value;

        Ok(InstructionPointer::Next)
    }

    // decrement the data pointer by one (move left)
    #[inline]
    fn execute_shl(&mut self) -> Result<InstructionPointer> {
        let (value, overflow) = self.dp.overflowing_sub(1);
        if overflow {
            return Err(Error::DataPointerOutOfBounds);
        }
        self.dp = value;

        Ok(InstructionPointer::Next)
    }

    // increment the byte at the data pointer by one modulo 256.
    #[inline]
    fn execute_inc(&mut self) -> Result<InstructionPointer> {
        let byte = self.read_byte()?;
        self.write_byte(byte.wrapping_add(1))?;

        Ok(InstructionPointer::Next)
    }

    // decrement the byte at the data pointer by one modulo 256.
    #[inline]
    fn execute_dec(&mut self) -> Result<InstructionPointer> {
        let byte = self.read_byte()?;
        self.write_byte(byte.wrapping_sub(1))?;

        Ok(InstructionPointer::Next)
    }

    // accept one byte of input, storing its value in the byte at the data pointer
    #[inline]
    fn execute_in(&mut self) -> Result<InstructionPointer> {
        let c = self
            .reader
            .try_write()
            .map_err(|_| Error::RwLock)?
            .read_one()?;
        self.write_byte(c)?;

        Ok(InstructionPointer::Next)
    }

    // output the byte at the data pointer
    #[inline]
    fn execute_out(&mut self) -> Result<InstructionPointer> {
        let byte = self.read_byte()?;
        let mut writer = self.writer.try_write().map_err(|_| Error::RwLock)?;
        writer.write_all(&[byte])?;
        writer.flush()?;

        Ok(InstructionPointer::Next)
    }

    // if the byte at the data pointer is zero, instead of moving the instruction pointer
    // forward to the next command, jump it forward to the command after the matching ']'
    // command
    fn execute_openbrk(&mut self) -> Result<InstructionPointer> {
        if self.read_byte()? == 0 {
            let matching_pos = find_matching(self.ip, &self.program)?;
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

// Given the position of an opening bracket (`pos`), it tries to find, in the `program`, the
// position of the matching closing bracket. It returns `Ok(usize)` if there is a matching closing
// bracket or `Err` otherwise.
fn find_matching(pos: usize, program: &[u8]) -> Result<usize> {
    let mut curr = pos + 1;
    let mut skipped = 0_u32;

    while curr < program.len() {
        if program[curr] == b']' {
            if skipped == 0 {
                return Ok(curr);
            } else {
                skipped -= 1;
            }
        } else if program[curr] == b'[' {
            skipped += 1;
        }

        curr += 1;
    }

    Err(Error::NoMatchingBracket(pos))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn increment_and_decrement_data_pointer() {
        let program = Arc::new([b'>', b'<']);
        let reader = Arc::new(RwLock::new(&b"hello"[..]));
        let writer = Arc::new(RwLock::new(Vec::new()));
        let mut machine = AbstractMachine::new(program, reader.clone(), writer.clone());
        machine.step().expect("valid operation >");
        assert_eq!(1, machine.dp);
        machine.step().expect("valid operation <");
        assert_eq!(0, machine.dp);
    }

    #[test]
    fn increment_and_decrement_byte_at_data_pointer() {
        let program = Arc::new([b'+', b'-']);
        let reader = Arc::new(RwLock::new(&b"hello"[..]));
        let writer = Arc::new(RwLock::new(Vec::new()));
        let mut machine = AbstractMachine::new(program, reader, writer);
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
        let program = Arc::new([b'[', b'+', b']', b'>']);
        let reader = Arc::new(RwLock::new(&b"hello"[..]));
        let writer = Arc::new(RwLock::new(Vec::new()));
        let mut machine = AbstractMachine::new(program, reader, writer).with_mem(vec![1, 2, 3]);
        machine.step().expect("valid operation");
        assert_eq!(machine.stack.last(), Some(&0));
    }

    #[test]
    fn skip_insert_existing_open_bracket_on_stack() {
        let program = Arc::new([b'[', b'+', b']', b'>']);
        let reader = Arc::new(RwLock::new(&b"hello"[..]));
        let writer = Arc::new(RwLock::new(Vec::new()));
        let mut machine = AbstractMachine::new(program, reader, writer)
            .with_mem(vec![1, 2, 3])
            .with_stack(vec![0]);
        machine.step().expect("valid operation");
        assert_eq!(machine.stack.last(), Some(&0));
        assert_eq!(1, machine.stack.len());
    }

    #[test]
    fn jump_to_instruction_after_matching_open_bracket() {
        let program = Arc::new([b'[', b'+', b']', b'>']);
        let reader = Arc::new(RwLock::new(&b"hello"[..]));
        let writer = Arc::new(RwLock::new(Vec::new()));
        let mut machine = AbstractMachine::new(program, reader, writer)
            .with_mem(vec![1, 2, 3])
            .with_stack(vec![0]);
        machine.ip = 2; // ip points to ']'
        machine.step().expect("valid operation");
        // instruction pointer points at the instruction after '['
        assert_eq!(1, machine.ip);
    }
}
