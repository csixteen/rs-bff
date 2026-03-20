use std::io::{self, Read, Write};

use bff_core::{ReadOne, Result};
use termios::{ECHO, ICANON, TCSANOW, Termios, tcsetattr};

pub struct TermiosReader;

impl ReadOne for TermiosReader {
    fn read_one(&mut self) -> Result<u8> {
        let mut buffer = [0_u8; 1];
        let fd = 0_i32; // stdin
        // Fetch the current termios struct, so that we can restore once we're done.
        let curr_termios = Termios::from_fd(fd)?;

        {
            // Copy the current termios, set the flags we're interested in, and then apply
            let mut new_termios = curr_termios;
            // We're doing two things here:
            // * disabling canonical mode
            // * disabling echoing input characters
            //
            // Canonical mode is set by default. In canonical mode, the input is made available line
            // by line, when the line delimiter is inserted. Except for EOL, the line
            // delimiter is included in the buffer returned by read. We don't want that.
            // In noncanonical mode, the input is made available immediately.
            new_termios.c_lflag &= !(ICANON | ECHO);
            // Set the parameters associated with the terminal from the new_termios struct. The flag
            // TCSANOW means that changes are effective immediately.
            tcsetattr(fd, TCSANOW, &new_termios)?;
            let mut reader = io::stdin();
            io::stdout().flush()?;
            reader.read_exact(&mut buffer)?;
        }

        // Restore stdin with original values of termios struct
        tcsetattr(fd, TCSANOW, &curr_termios)?;

        Ok(buffer[0])
    }
}
