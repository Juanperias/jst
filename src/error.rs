use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum JstError {
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Cannot Fork")]
    ForkError,

    #[error("Process exited with {0}")]
    ProcExited(i32),

    #[error("Invalid Syscall {0}")]
    InvalidSyscall(u64),
    
    #[error("Cannot find {0}")]
    CannotFindPath(PathBuf),
}
