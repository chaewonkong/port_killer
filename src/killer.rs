use std::fmt;
use sysinfo::{Pid, System};

#[derive(Debug)]
pub enum KillError {
    ProcessNotFound(u32),
    PermissionDenied(u32),
}

impl std::error::Error for KillError {}

impl fmt::Display for KillError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KillError::ProcessNotFound(pid) => {
                return write!(
                    f,
                    "PID {} 프로세스를 찾을 수 없습니다 (이미 종료되었을 수 있음)",
                    pid
                );
            }
            KillError::PermissionDenied(pid) => {
                return write!(
                    f,
                    "PID {} 프로세스를 종료할 권한이 부족합니다 (sudo 필요)",
                    pid
                );
            }
        }
    }
}

pub fn port_killer(pid: u32) -> Result<(), KillError> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let target_pid = Pid::from(pid as usize);

    match sys.process(target_pid) {
        Some(process) if process.kill() => Ok(()),
        Some(_) => Err(KillError::PermissionDenied(pid)),
        None => Err(KillError::ProcessNotFound(pid)),
    }
}
