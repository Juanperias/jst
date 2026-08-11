use std::{path::PathBuf, str::FromStr};

use crate::{error::JstError, mem::MemReader, ptrace::create_ptrace_proc, syscall::Syscall};
use clap::Parser;

mod mem;
mod ptrace;
mod syscall;
mod error;

#[derive(Parser)]
struct Args {
    file: PathBuf,
}

fn path_resolve(search: PathBuf) -> Option<PathBuf> {
    let path = match std::env::var("PATH") {
        Ok(v) => v,
        Err(_) => return None,
    };

    for key in path.split(":") {
        let v = format!("{key}/{}", search.to_str().unwrap_or_default());
        let p = match PathBuf::from_str(v.as_str()) {
            Ok(p) => p,
            Err(_) => continue,
        };

        if p.exists() {
            return Some(p);
        }
    }

    None
}

fn main() -> Result<(), JstError> {
    let args = Args::parse();

    let path = {
        if args.file.exists() {
            args.file.clone()
        } else {
            match path_resolve(args.file.clone()) {
                Some(p) => p,
                None => return Err(JstError::CannotFindPath(args.file)),
            }
        }
    };

    let proc = create_ptrace_proc(path, vec![], vec![])?;
    
    let mut mem_reader = MemReader::new(proc.0)?;

    loop {
        if let Err(err) = proc.syscall() {
            println!("{}", err);
            break;
        }

        let regs = proc.regs()?;

        if let Ok(syscall) = Syscall::new(regs, &mut mem_reader) {
            println!("{syscall}");
        }
    }

    Ok(())
}
