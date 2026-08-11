use std::{fmt::Display, fs::File, io::{Read, Seek}};

use crate::{error::JstError, mem::{MemBuf, MemReader}};

#[derive(Debug, Clone)]
pub enum Syscall {
    SysRead {
        fd: u64,
        buf: MemBuf,
        count: usize,
        ret: u64,
    },
    SysWrite {
        fd: u64,
        buf: MemBuf,
        count: usize,
        ret: u64,
    },
    SysOpen {
        filename: MemBuf,
        flags: i32,
        mode: i32,
        ret: u64,
    },
    SysBrk {
        brk: MemBuf,
        ret: u64,
    },
    SysClose {
        fd: i32,
        ret: i32,
    },
    SysExit(u64),
}

impl Syscall {
    pub fn new(regs: libc::user_regs_struct, mem_reader: &mut MemReader) -> Result<Self, JstError> {
    
        match regs.orig_rax {
            0 => {
                if regs.rax > regs.rdx {
                    return Err(JstError::InvalidSyscall(0));
                }

                Ok(Self::SysRead {
                    fd: regs.rdi,
                    buf: MemBuf::Addr(regs.rsi),
                    count: regs.rdx as usize,
                    ret: regs.rax,
                })
            },
            1 => {
                if regs.rax > regs.rdx {
                    return Err(JstError::InvalidSyscall(0));
                }

                let buf = mem_reader.read(regs.rsi, regs.rdx as usize)?;

                Ok(Self::SysWrite {
                    fd: regs.rdi,
                    buf: buf,
                    count: regs.rdx as usize,
                    ret: regs.rax,
                })
            },
            2 => {
                let buf = mem_reader.read_null_terminated(regs.rdi)?;

                Ok(Self::SysOpen { 
                    filename: buf, 
                    flags: regs.rsi as i32, 
                    mode: regs.rdx as i32,
                    ret: regs.rax,
                })
            },
            3 => {
                Ok(Self::SysClose {
                    fd: regs.rdi as i32,
                    ret: regs.rax as i32,
                })
            },
            12 => {
                let addr = MemBuf::from_addr(regs.rdi);

                Ok(Self::SysBrk { brk: addr, ret: regs.rax })
            },
            60 => {
                Ok(Self::SysExit(regs.rdi))
            },
            n => {
                Err(JstError::InvalidSyscall(n))
            },
        }
    }
}

impl Display for Syscall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SysRead { fd, buf, count, ret } => {
                write!(f, "read({fd}, {buf}, {count}) = {ret}")
            },
            Self::SysWrite { fd, buf, count, ret } => {
                write!(f, "write({fd}, {buf}, {count}) = {ret}")
            },
            Self::SysOpen { filename, flags, mode, ret } => {
                write!(f, "open({filename}, {flags}, {mode}) = {ret}")
            },
            Self::SysClose { fd, ret } => {
                write!(f, "close({fd}) = {ret}")
            },
            Self::SysBrk { brk, ret } => {
                write!(f, "brk({:x}) = 0x{:x}", brk, ret)
            },
            Self::SysExit(code) => {
                write!(f, "exit({code})")
            }
        }
    }
}
