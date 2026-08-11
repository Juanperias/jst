use std::{ffi::CString, path::PathBuf, str::FromStr};

use libc::{WEXITSTATUS, WIFEXITED, waitpid};

use crate::error::JstError;

pub struct PtraceProc(pub i32);

impl PtraceProc {
    pub fn syscall(&self) -> Result<(), JstError> {
        let status = self.raw_call(libc::PTRACE_SYSCALL, 0, 0)?;

        if status != 0 {
            return Err(JstError::IoError(std::io::Error::last_os_error()));
        }

        self.wait()?;

        Ok(())
    }

    pub fn regs(&self) -> Result<libc::user_regs_struct, JstError> {
        let mut regs: libc::user_regs_struct = unsafe { std::mem::zeroed() } ;

        let status = self.raw_call(libc::PTRACE_GETREGS, 0, (&mut regs as *mut libc::user_regs_struct).addr() as u64)?;
            
        if status != 0 {
            return Err(JstError::IoError(std::io::Error::last_os_error()));
        }

        Ok(regs)

    }

    pub fn raw_call(&self, request: u32, addr: u64, data: u64) -> Result<i64, JstError> {
        unsafe {
            *libc::__errno_location() = 0;

            let val = libc::ptrace(request, self.0, addr, data);

            if *libc::__errno_location() != 0 {
                return Err(JstError::IoError(std::io::Error::last_os_error()));
            }

            Ok(val)
        }
        
    }

    pub fn wait(&self) -> Result<(), JstError> {
        let mut status = 0;

        unsafe {
            waitpid(self.0, &mut status as *mut i32, 0);

            if libc::WIFEXITED(status) {
                return Err(JstError::ProcExited(WEXITSTATUS(status)));
            }

            Ok(())
        }
    }
}

pub fn create_ptrace_proc(path: PathBuf, argv: Vec<String>, envp: Vec<String>) -> Result<PtraceProc, JstError> {
    let path = path.to_str().unwrap_or_default();

    let pid = unsafe { libc::fork() };
    
    if pid < 0 {
        return Err(JstError::IoError(std::io::Error::last_os_error()));     
    }

    if pid == 0 {
        unsafe {
            libc::ptrace(libc::PTRACE_TRACEME);

            let c_path = CString::from_str(&path).unwrap();

            let argv = vec![
                c_path.as_ptr(),
                core::ptr::null()
            ];
        
            libc::execve(c_path.as_ptr(), argv.as_ptr(), core::ptr::null_mut());


            libc::perror(c"Cannot create traceme process... ".as_ptr());
            
            libc::exit(1);
        }
    }
    
    let mut status = 0;

    unsafe {
        waitpid(pid, &mut status as *mut i32, 0);

        if WIFEXITED(status) {
            return Err(JstError::ForkError);
        }
    }

    Ok(PtraceProc(pid))
}
