use kv::Error as KVError;
use magic_crypt::MagicCryptError as MagicError;
use std::error::Error as StdError;
use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult,
};
use std::io::Error as IoError;
use std::result::Result as StdResult;

#[derive(Debug)]
pub(crate) enum NotpError {
    Generic(String),

    Io(IoError),

    Kv(KVError),

    McError(MagicError),
}

use NotpError::*;

impl Display for NotpError {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> FmtResult {
        match self {
            Generic(ref string) => write!(f, "Generic error: {}", string),
            Kv(ref err) => write!(f, "SecretStore error: {}", err),
            Io(ref err) => write!(f, "I/O error: {}", err),
            McError(ref err) => {
                write!(f, "Encryption/Decryption error: {}", err)
            }
        }
    }
}

impl StdError for NotpError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Kv(ref err) => Some(err),
            Io(ref err) => Some(err),
            McError(ref err) => Some(err),
            Generic(_) => None,
        }
    }
}

impl From<KVError> for NotpError {
    fn from(err: KVError) -> Self {
        Kv(err)
    }
}

impl From<IoError> for NotpError {
    fn from(err: IoError) -> Self {
        Io(err)
    }
}

impl From<MagicError> for NotpError {
    fn from(err: MagicError) -> Self {
        McError(err)
    }
}

impl From<String> for NotpError {
    fn from(err: String) -> Self {
        Generic(err)
    }
}

pub(crate) type NotpResult<T> = StdResult<T, NotpError>;
