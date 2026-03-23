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
    #[error("could not acquire rw lock")]
    RwLock,
}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;
