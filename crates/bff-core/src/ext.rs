use std::io;

use super::Result;

pub trait ReadOne {
    fn read_one(&mut self) -> Result<u8>;
}

impl<T> ReadOne for T
where
    T: io::Read,
{
    fn read_one(&mut self) -> Result<u8> {
        let mut buf = [0];
        self.read_exact(&mut buf)?;

        Ok(buf[0])
    }
}
