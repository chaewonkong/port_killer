use inquire::{Confirm, Select};
use port_killer::{killer::kill, parser::parse_port_process_lookup, prompter::PortItem};
use std::process::Command;

fn main() {
    let output = Command::new("lsof")
        .args(["-iTCP", "-sTCP:LISTEN", "-P", "-n", "+c", "0"])
        .output();

    if let Ok(out) = output {
        match parse_port_process_lookup(&out) {
            Ok(map) => {
                // HashMap -> Vec<PortItem>
                let mut items: Vec<PortItem> = map
                    .into_iter()
                    .map(|(port, process)| PortItem {
                        port,
                        name: process.name,
                        pid: process.id,
                    })
                    .collect();

                items.sort_by_key(|item| item.port);

                // print prompt
                let selection =
                    Select::new("종료할 포트/서비스를 선택하세요 (↑↓, Enter):", items).prompt();

                match selection {
                    Ok(target) => {
                        let prompt_msg = format!(
                            "정말 포트 {} ({}) 프로세스(PID: {})를 Kill 하시겠습니까?",
                            target.port, target.name, target.pid
                        );

                        if let Ok(true) = Confirm::new(&prompt_msg).with_default(false).prompt() {
                            match kill(&target) {
                                Ok(()) => println!(
                                    "\n✅ 포트 {} ({}) 프로세스(PID: {})를 성공적으로 종료했습니다.",
                                    target.port, target.name, target.pid
                                ),
                                Err(err) => eprintln!("\n❌ 프로세스 종료 실패: {}", err),
                            }
                        } else {
                            println!("\n🚫 작업이 취소되었습니다.");
                        }
                    }
                    Err(_) => println!("\n선택이 취소되었습니다."),
                }
            }
            Err(err) => {
                eprintln!("❌ 포트 정보를 파싱하지 못했습니다: {:?}", err);
            }
        }
    }
}
