use port_killer::parser::parse_port_process_lookup;
use std::process::Command;

fn main() {
    let output = Command::new("lsof")
        .args(["-iTCP", "-sTCP:LISTEN", "-P", "-n", "+c", "0"])
        .output();

    if let Ok(out) = output {
        match parse_port_process_lookup(&out) {
            Ok(map) => {
                for (port, process) in &map {
                    println!("{:<8} | {:<30} | {}", port, process.name, process.id)
                }
            }
            Err(err) => {
                eprintln!("❌ 포트 정보를 파싱하지 못했습니다: {:?}", err);
            }
        }
    }
}
