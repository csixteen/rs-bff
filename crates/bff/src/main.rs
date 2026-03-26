mod reader;

use std::{
    fs,
    io::{self, Read},
    path::PathBuf,
    sync::{Arc, RwLock},
};

use bff_core::AbstractMachine;
use clap::{Parser, ValueHint};

use self::reader::TermiosReader;

/// Brainfuck interpreter
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of memory cells that the abstract machine will operate on
    #[arg(short, long, default_value_t = AbstractMachine::DEFAULT_NUM_CELLS)]
    cells: usize,

    #[arg(short, long, value_hint = ValueHint::FilePath)]
    file: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let Args { cells, file } = Args::parse();
    let program: Arc<[u8]> = read_program(file)?.into();
    let reader = Arc::new(RwLock::new(TermiosReader));
    let writer = Arc::new(RwLock::new(io::stdout()));
    let mut machine = AbstractMachine::new(program, reader, writer).with_num_cells(cells);

    if let Err(e) = machine.run() {
        eprintln!("{}", e);
        return Err(e.into());
    }

    Ok(())
}

// Reads the program from `file`, if it's `Some(path)`, or from STDIN otherwise.
fn read_program(file: Option<PathBuf>) -> anyhow::Result<Vec<u8>> {
    match file {
        Some(f) => Ok(fs::read(f)?),
        None => {
            let mut buffer = Vec::new();
            let mut stdin = io::stdin();
            stdin.read_to_end(&mut buffer)?;
            Ok(buffer)
        }
    }
}
