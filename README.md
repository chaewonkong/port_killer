# 🎯 Interactive Port Killer

터미널에서 현재 사용 중인 포트와 프로세스를 스캔하고, 방향키로 선택하여 간편하게 강제 종료(Kill)할 수 있는 인터랙티브 CLI 유틸리티입니다.

매번 `lsof -i :8080`을 치고 PID를 확인한 후 `kill -9 <PID>`를 입력하던 번거로운 수동 과정을 우아하게 자동화하기 위해 설계된 가벼운 개발자 도구입니다.

## ✨ 주요 기능 (Features)

- **자동 포트 스캔:** 현재 LISTEN 상태인 TCP 포트와 해당 포트를 점유하고 있는 프로세스 정보(이름, PID)를 자동으로 탐색합니다.
- **직관적인 TUI (인터랙티브 UI):** 복잡한 인수 입력 없이 `위/아래 방향키`로 종료 대상 서비스를 손쉽게 선택할 수 있습니다.
- **안전한 확인 절차:** 프로세스를 즉시 종료하지 않고, 최종 확인(`y/N`) 프롬프트를 제공하여 오작동 및 실수를 사전에 방지합니다.

## 🚀 시작하기 (Getting Started)

### 사전 요구 사항 (Prerequisites)

- **macOS** 또는 **Linux** 환경을 권장합니다. (시스템 포트 조회를 위해 내부적으로 `lsof` 명령어를 활용합니다)

### 설치 (Installation)

```bash
curl -sSf https://raw.githubusercontent.com/owner/port_killer/main/install.sh | sh
```


## 💻 사용법 (Usage)

터미널에서 아래 명령어를 실행하여 프로그램을 시작합니다.

```bash
portkill
```

> **💡 권한 관련 팁 (Tip):** 시스템 루트 권한(예: 웹 서버 포트 80, 443 등)으로 실행 중인 특정 프로세스를 조회하고 안전하게 종료하려면 `sudo` 권한이 필요할 수 있습니다. (`sudo portkill`)

### 🕹️ 조작 가이드

- `↑` / `↓` (방향키): 실행 중인 포트/서비스 목록 이동
- `Enter` (엔터키): 타겟 서비스 선택 및 프로세스 종료(Kill) 확인 프롬프트 진입
- `y` (Yes) / `n` (No): 선택한 프로세스 실제 종료 여부 결정
- `Esc` 또는 `Ctrl + C`: 프로그램 즉시 취소 및 종료

## 🛠️ 기술 스택 (Tech Stack)

- **Language:** [Rust](https://www.rust-lang.org/) (안전하고 예측 가능한 초고속 런타임 성능)
- **Interactive UI:** [inquire](https://crates.io/crates/inquire) (인터랙티브 CLI 프롬프트 및 방향키 선택 인터페이스 제어)
- **Process Management:** [sysinfo](https://crates.io/crates/sysinfo) (플랫폼 독립적인 프로세스 정보 추적 및 시그널 전송)

## 📄 라이선스 (License)

이 프로젝트는 MIT License를 따릅니다. 자유롭게 수정하여 개인의 개발 워크플로우에 최적화해 보세요!
