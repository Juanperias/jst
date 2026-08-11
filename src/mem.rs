use std::{fmt::{Display, LowerHex}, fs::File, io::{Read, Seek}, path::PathBuf};

use clap::error::Result;

use crate::error::JstError;

#[derive(Debug)]
pub struct MemReader(Option<File>);

impl MemReader {
    pub fn new(pid: i32) -> Result<Self, JstError> {
        let path = PathBuf::from(format!("/proc/{pid}/mem"));
        let file = File::open(path).ok();

        Ok(Self(file))
    }
    pub fn read(&mut self, addr: u64, size: usize) -> Result<MemBuf, JstError> {
        if let Some(file) = &mut self.0 {
            let _ = file.seek(std::io::SeekFrom::Start(addr));
            
            let mut buf = vec![0_u8; size];

            file.read_exact(&mut buf)?;

            return Ok(match String::from_utf8(buf) {
                Ok(s) => MemBuf::Str(s),
                Err(_) => MemBuf::Addr(addr),
            });

        }

        Ok(MemBuf::Addr(addr))
    }
    pub fn read_null_terminated(&mut self, addr: u64) -> Result<MemBuf, JstError> {
        if let Some(file) = &mut self.0 {
            let _ = file.seek(std::io::SeekFrom::Start(addr));

            let mut string = String::new();

            while let Ok(n) = read_byte(file) {
                if n == 0 {
                    break;
                }

                string.push(n as char);
            }

            return Ok(MemBuf::Str(string));
        }

        Ok(MemBuf::Addr(addr))
    }
}

fn read_byte(f: &mut File) -> Result<u8, JstError> {
    let mut b = vec![0_u8; 1];

    f.read_exact(&mut b)?;

    Ok(b[0])
}

#[derive(Clone, Debug)]
pub enum MemBuf {
    Addr(u64),
    Str(String),
    Null,
}

impl LowerHex for MemBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Addr(n) => {
                write!(f, "0x{:x}", n)
            },
            Self::Null => {
                write!(f, "NULL")
            },
            Self::Str(str) => {
                let addr =  str.as_ptr().addr();

                write!(f, "0x{:x}", addr)
            },
        }
    }
}

impl MemBuf {
    pub fn from_addr(addr: u64) -> Self {
        match addr {
            0 => Self::Null,
            n => Self::Addr(n)
        }
    }
}

impl Display for MemBuf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Addr(a) => write!(f, "{a}"),
            Self::Str(s) => write!(f, "{:?}", s),
            Self::Null => write!(f, "NULL"),
        }
    }
}
