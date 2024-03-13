use core::num;
use std::{fmt::Debug, io};

pub type Result<T, E = IDTapError> = ::core::result::Result<T, E>;

pub enum IDTapError {
    IO(io::Error),
    ParseInt(std::num::ParseIntError),
    RuSql(rusqlite::Error),
    SerialPort(serialport::Error),
}

impl Debug for IDTapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(ioe) => write!(f, "I/O Error: {ioe:?}"),
            Self::ParseInt(pie) => write!(f, "Int Parse Error: {pie:?}"),
            Self::RuSql(rse) => write!(f, "RuSQLite Error: {rse:?}"),
            Self::SerialPort(spe) => write!(f, "SerialPort Error: {spe:?}"),
        }
    }
}

impl From<io::Error> for IDTapError {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<num::ParseIntError> for IDTapError {
    fn from(value: num::ParseIntError) -> Self {
        Self::ParseInt(value)
    }
}

impl From<rusqlite::Error> for IDTapError {
    fn from(value: rusqlite::Error) -> Self {
        Self::RuSql(value)
    }
}

impl From<serialport::Error> for IDTapError {
    fn from(value: serialport::Error) -> Self {
        Self::SerialPort(value)
    }
}
