use std::sync::TryLockError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("should this happen?")]
    AbstractMachineMissing,

    #[error(transparent)]
    BffCore(#[from] bff_core::Error),

    #[error(transparent)]
    Io(#[from] ::std::io::Error),

    #[error("couldn't acquire lock")]
    Lock,
}

impl<T> From<TryLockError<T>> for Error {
    fn from(_value: TryLockError<T>) -> Self {
        Self::Lock
    }
}

pub type Result<T, E = Error> = ::std::result::Result<T, E>;
