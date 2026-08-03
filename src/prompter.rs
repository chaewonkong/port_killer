use std::fmt;

#[derive(Clone)]
pub struct PortItem {
    pub port: u16,
    pub name: String,
    pub pid: u32,
}

// 터미널 목록에 "포트 8080 | node (PID: 1234)" 형태로 표시
impl fmt::Display for PortItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(
            f,
            "포트 {:<5} | {:<25} (PID: {})",
            self.port, self.name, self.pid
        );
    }
}
