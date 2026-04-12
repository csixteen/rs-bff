mod error;
mod ext;

use std::{
    collections::HashMap,
    io, ptr,
    sync::{Arc, RwLock},
};

pub use self::{error::*, ext::*};

pub type Reader<'a> = Arc<RwLock<dyn ReadOne + 'a>>;
pub type Writer<'a> = Arc<RwLock<dyn io::Write + 'a>>;

#[cfg(feature = "thread-safe")]
pub struct AbstractMachine<'a> {
    // Data pointer, indicating the current cell being pointed at.
    dp: usize,
    // The one-dimensional tape of memory cells that the Brainfuck program operates.
    mem: Vec<u8>,
    // Instruction pointer, which points to the next command to be executed.
    ip: usize,
    // The actual Brainfuck source code that we're running.
    program: Arc<[u8]>,
    // A reader where the input command will read from.
    reader: Reader<'a>,
    // A writer where the output command will write onto.
    writer: Writer<'a>,
    brackets: HashMap<usize, usize>,
}

#[cfg(not(feature = "thread-safe"))]
pub struct AbstractMachine<'a> {
    dp: usize,
    mem: Vec<u8>,
    ip: usize,
    program: Arc<[u8]>,
    reader: &'a mut dyn ReadOne,
    writer: &'a mut dyn io::Write,
    brackets: HashMap<usize, usize>,
}

#[cfg(not(feature = "thread-safe"))]
impl<'a> AbstractMachine<'a> {
    pub const DEFAULT_NUM_CELLS: usize = 30_000;

    /// Creates a new Brainfuck abstract machine to run the given program, a reader and a writer.
    pub fn new(
        program: Arc<[u8]>,
        reader: &'a mut dyn ReadOne,
        writer: &'a mut dyn io::Write,
    ) -> Result<Self> {
        let brackets = build_bracket_mapping(&program)?;

        Ok(Self {
            dp: 0,
            mem: vec![0_u8; Self::DEFAULT_NUM_CELLS],
            ip: 0,
            program,
            reader,
            writer,
            brackets,
        })
    }
}

#[cfg(feature = "thread-safe")]
impl<'a> AbstractMachine<'a> {
    pub const DEFAULT_NUM_CELLS: usize = 30_000;

    /// Creates a new Brainfuck abstract machine to run the given program, a reader and a writer.
    pub fn new(program: Arc<[u8]>, reader: Reader<'a>, writer: Writer<'a>) -> Result<Self> {
        let brackets = build_bracket_mapping(&program)?;

        Ok(Self {
            dp: 0,
            mem: vec![0_u8; Self::DEFAULT_NUM_CELLS],
            ip: 0,
            program,
            reader,
            writer,
            brackets,
        })
    }
}

impl<'a> AbstractMachine<'a> {
    /// Given an abstract machine, it initializes its memory with [`num_cells`] set to zero.
    pub fn with_num_cells(mut self, num_cells: usize) -> Self {
        self.mem = vec![0_u8; num_cells];
        self
    }

    pub fn with_program(mut self, program: Arc<[u8]>) -> Self {
        self.program = program;
        self
    }

    #[cfg(test)]
    fn with_mem(mut self, mem: Vec<u8>) -> Self {
        self.mem = mem;
        self
    }

    pub fn restart(&mut self) {
        self.dp = 0;
        self.ip = 0;

        unsafe {
            let capacity = self.mem.capacity();
            let mem_ptr = self.mem.as_mut_ptr();
            ptr::write_bytes(mem_ptr, 0x0, capacity);
        }
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

        match command {
            b'>' => self.execute_shr()?,
            b'<' => self.execute_shl()?,
            b'+' => self.execute_inc()?,
            b'-' => self.execute_dec()?,
            b'.' => self.execute_out()?,
            b',' => self.execute_in()?,
            b'[' if self.read_byte()? == 0 => {
                self.ip = *self
                    .brackets
                    .get(&self.ip)
                    .ok_or(Error::NoMatchingBracket(self.ip))?;
            }
            b']' if (self.read_byte()? != 0) => {
                self.ip = *self
                    .brackets
                    .get(&self.ip)
                    .ok_or(Error::NoMatchingBracket(self.ip))?;
            }
            _ => (),
        };

        self.ip += 1;

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
    fn execute_shr(&mut self) -> Result<()> {
        let (value, overflow) = self.dp.overflowing_add(1);
        if overflow {
            return Err(Error::DataPointerOutOfBounds);
        }
        self.dp = value;

        Ok(())
    }

    // decrement the data pointer by one (move left)
    #[inline]
    fn execute_shl(&mut self) -> Result<()> {
        let (value, overflow) = self.dp.overflowing_sub(1);
        if overflow {
            return Err(Error::DataPointerOutOfBounds);
        }
        self.dp = value;

        Ok(())
    }

    // increment the byte at the data pointer by one modulo 256.
    #[inline]
    fn execute_inc(&mut self) -> Result<()> {
        let byte = self.read_byte()?;
        self.write_byte(byte.wrapping_add(1))?;

        Ok(())
    }

    // decrement the byte at the data pointer by one modulo 256.
    #[inline]
    fn execute_dec(&mut self) -> Result<()> {
        let byte = self.read_byte()?;
        self.write_byte(byte.wrapping_sub(1))?;

        Ok(())
    }

    #[cfg(not(feature = "thread-safe"))]
    // accept one byte of input, storing its value in the byte at the data pointer
    #[inline]
    fn execute_in(&mut self) -> Result<()> {
        let c = self.reader.read_one()?;
        self.write_byte(c)?;

        Ok(())
    }

    #[cfg(feature = "thread-safe")]
    // accept one byte of input, storing its value in the byte at the data pointer
    #[inline]
    fn execute_in(&mut self) -> Result<()> {
        let c = self
            .reader
            .try_write()
            .map_err(|_| Error::RwLock)?
            .read_one()?;
        self.write_byte(c)?;

        Ok(())
    }

    #[cfg(not(feature = "thread-safe"))]
    // output the byte at the data pointer
    #[inline]
    fn execute_out(&mut self) -> Result<()> {
        let byte = self.read_byte()?;
        self.writer.write_all(&[byte])?;
        self.writer.flush()?;

        Ok(())
    }

    #[cfg(feature = "thread-safe")]
    // output the byte at the data pointer
    #[inline]
    fn execute_out(&mut self) -> Result<()> {
        let byte = self.read_byte()?;
        let mut writer = self.writer.try_write().map_err(|_| Error::RwLock)?;
        writer.write_all(&[byte])?;
        writer.flush()?;

        Ok(())
    }

    pub fn to_debug_info(&self) -> DebugInfo {
        DebugInfo {
            data_pointer: self.dp,
            current_cell: self.mem[self.dp],
            instruction_pointer: self.ip,
            current_instruction: self.program[self.ip],
        }
    }
}

fn build_bracket_mapping(program: &[u8]) -> Result<HashMap<usize, usize>> {
    let mut stack = Vec::new();
    let mut res = HashMap::new();

    for (pos, &b) in program.iter().enumerate() {
        if b == b'[' {
            stack.push(pos);
        } else if b == b']' {
            let open = stack.pop().ok_or(Error::NoMatchingBracket(pos))?;
            res.insert(open, pos);
            res.insert(pos, open);
        }
    }

    Ok(res)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugInfo {
    pub data_pointer: usize,
    pub current_cell: u8,
    pub instruction_pointer: usize,
    pub current_instruction: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! build_and_test {
        ($program:expr, $machine:ident, $body:tt) => {
            #[cfg(feature = "thread-safe")]
            {
                let reader = Arc::new(RwLock::new(&b"hello"[..]));
                let writer = Arc::new(RwLock::new(Vec::new()));
                let mut $machine =
                    AbstractMachine::new($program, reader, writer).expect("valid program");

                $body
            }

            #[cfg(not(feature = "thread-safe"))]
            {
                let mut reader = &b"hello"[..];
                let mut writer = Vec::new();
                let mut $machine = AbstractMachine::new($program, &mut reader, &mut writer)
                    .expect("valid program");

                $body
            }
        };
    }

    #[test]
    fn increment_and_decrement_data_pointer() {
        let program = Arc::new([b'>', b'<']);
        build_and_test!(program, machine, {
            machine.step().expect("valid operation >");
            assert_eq!(1, machine.dp);
            machine.step().expect("valid operation <");
            assert_eq!(0, machine.dp);
        });
    }

    #[test]
    fn increment_and_decrement_byte_at_data_pointer() {
        let program = Arc::new([b'+', b'-']);
        build_and_test!(program, machine, {
            machine.step().expect("valid operation +");
            assert_eq!(1, machine.mem[0]);
            machine.step().expect("valid operation -");
            assert_eq!(0, machine.mem[0]);
        });
    }

    #[test]
    fn jump_to_instruction_after_matching_open_bracket() {
        let program = Arc::new([b'[', b'+', b']', b'>']);
        build_and_test!(program, machine, {
            machine = machine.with_mem(vec![1, 2, 3]);
            machine.ip = 2; // ip points to ']'
            machine.step().expect("valid operation");
            // instruction pointer points at the instruction after '['
            assert_eq!(1, machine.ip);
        });
    }
}
