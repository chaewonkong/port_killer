use std::{fmt, process::Command};
use sysinfo::{Pid, System};

use crate::prompter::PortItem;

#[derive(Debug)]
pub enum KillError {
    ProcessNotFound(u32),
    PermissionDenied(u32),
    DockerCommandFailed(String),
    DockerContainerKillFailed(String),
    DockerContainerNotFound(u16),
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
            KillError::DockerCommandFailed(msg) => {
                return write!(f, "Docker 명령어가 실패했습니다: {}", msg);
            }
            KillError::DockerContainerKillFailed(container_id) => {
                return write!(
                    f,
                    "Docker container {} 의 종료가 실패했습니다",
                    container_id
                );
            }
            KillError::DockerContainerNotFound(port) => {
                return write!(
                    f,
                    "Port:{} 를 사용하는 Docker container를 찾을 수 없습니다",
                    port
                );
            }
        }
    }
}

pub fn kill(item: &PortItem) -> Result<(), KillError> {
    if item.name == "com.docker.backend" {
        stop_docker_container_by_port(item.port)
    } else {
        kill_process_by_pid(item.pid)
    }
}

fn stop_docker_container_by_port(port: u16) -> Result<(), KillError> {
    let output = Command::new("docker")
        .args(["ps", "--format", "{{.ID}}\t{{.Ports}}"])
        .output()
        .map_err(|e| KillError::DockerCommandFailed(e.to_string()))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines() {
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() >= 2 {
            let container_id = parts[0];
            let ports_str = parts[1];

            if is_host_port_mapped(ports_str, port) {
                let stop_status = Command::new("docker")
                    .args(["stop", container_id])
                    .status()
                    .map_err(|e| KillError::DockerCommandFailed(e.to_string()))?;

                if stop_status.success() {
                    return Ok(());
                } else {
                    return Err(KillError::DockerContainerKillFailed(
                        container_id.to_string(),
                    ));
                }
            }
        }
    }

    Err(KillError::DockerContainerNotFound(port))
}

fn kill_process_by_pid(pid: u32) -> Result<(), KillError> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let target_pid = Pid::from(pid as usize);

    match sys.process(target_pid) {
        Some(process) if process.kill() => Ok(()),
        Some(_) => Err(KillError::PermissionDenied(pid)),
        None => Err(KillError::ProcessNotFound(pid)),
    }
}

fn is_host_port_mapped(ports_raw: &str, target_port: u16) -> bool {
    // 쉼표(,) 및 공백 단위로 각 포트 매핑 항목을 분리
    // 예: "0.0.0.0:2353->2345/tcp", "[::]:2353->2345/tcp" ...
    for mapping in ports_raw.split(',') {
        let mapping = mapping.trim();

        // '->'가 있는 매핑 정보만 처리 (호스트에 노출된 포트)
        if let Some((host_port, _container_port)) = mapping.split_once("->") {
            // host_part 예시: "0.0.0.0:8001" 또는 "[::]:8001" 또는 ":::8001"
            // 오른쪽 끝에서부터 ':' 기준으로 분리하여 포트 번호 획득
            if let Some(port_str) = host_port.rsplit(':').next() {
                if let Ok(host_port) = port_str.parse::<u16>() {
                    if host_port == target_port {
                        return true; // 호스트 포트 일치 확인!
                    }
                }
            }
        }
    }

    false
}
