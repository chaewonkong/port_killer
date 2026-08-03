use std::collections::HashMap;
use std::process::Output;

pub struct Process {
    pub name: String,
    pub id: u32,
}

#[derive(Debug)]
pub enum ParseError {
    InvalidUtf8,
    NoPortsFound,
    ParsingFailed(String),
}

pub fn parse_port_process_lookup(out: &Output) -> Result<HashMap<u16, Process>, ParseError> {
    let mut map: HashMap<u16, Process> = HashMap::new();

    let stdout = String::from_utf8_lossy(&out.stdout);
    for line in stdout.lines().skip(1) {
        // parts[0]: 프로세스 이름 ("node")
        // parts[1]: PID ("12345")
        // parts[8]: 주소 및 포트 정보 ("*:8080" 또는 "127.0.0.1:5432")
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 9 {
            continue;
        }

        let p_name = parts[0].replace("\\x20", " ");
        let pid: u32 = match parts[1].parse() {
            Ok(p) => p,
            Err(_) => continue,
        };

        if let Some(port_str) = parts[8].rsplit(':').next() {
            if let Ok(port) = port_str.parse::<u16>() {
                map.entry(port).or_insert(Process {
                    name: p_name,
                    id: pid,
                });
            }
        }
    }

    if map.is_empty() {
        return Err(ParseError::NoPortsFound);
    }

    Ok(map)
}
