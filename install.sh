#!/usr/bin/env sh
# portkill installer
# Usage:
#   curl -sSf https://raw.githubusercontent.com/<owner>/<repo>/main/install.sh | sh
#   curl -sSf .../install.sh | sh -s -- v0.2.0   (특정 버전 설치)

set -eu

# TODO: 실제 GitHub owner/repo로 바꾸세요.
REPO="owner/port_killer"
BINARY_NAME="portkill"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

VERSION="${1:-latest}"

err() {
  echo "error: $1" >&2
  exit 1
}

need_cmd() {
  command -v "$1" >/dev/null 2>&1 || err "필요한 명령어가 없습니다: $1"
}

need_cmd curl
need_cmd tar
need_cmd mktemp

detect_target() {
  os="$(uname -s)"
  arch="$(uname -m)"

  case "$os" in
    Linux)
      case "$arch" in
        x86_64) echo "x86_64-unknown-linux-gnu" ;;
        aarch64 | arm64) echo "aarch64-unknown-linux-gnu" ;;
        *) err "지원하지 않는 아키텍처: $arch ($os)" ;;
      esac
      ;;
    Darwin)
      case "$arch" in
        x86_64) echo "x86_64-apple-darwin" ;;
        arm64) echo "aarch64-apple-darwin" ;;
        *) err "지원하지 않는 아키텍처: $arch ($os)" ;;
      esac
      ;;
    *)
      err "지원하지 않는 OS: $os (Windows는 release 페이지에서 .zip을 받아 직접 설치하세요)"
      ;;
  esac
}

main() {
  target="$(detect_target)"

  if [ "$VERSION" = "latest" ]; then
    url="https://github.com/${REPO}/releases/latest/download/${BINARY_NAME}-${target}.tar.gz"
  else
    url="https://github.com/${REPO}/releases/download/${VERSION}/${BINARY_NAME}-${target}.tar.gz"
  fi

  tmpdir="$(mktemp -d)"
  trap 'rm -rf "$tmpdir"' EXIT

  echo "다운로드 중: $url"
  curl -sSfL "$url" -o "$tmpdir/${BINARY_NAME}.tar.gz" \
    || err "다운로드 실패. 버전/플랫폼($target)에 맞는 릴리스가 있는지 확인하세요."

  tar -xzf "$tmpdir/${BINARY_NAME}.tar.gz" -C "$tmpdir"

  if [ -w "$INSTALL_DIR" ]; then
    mv "$tmpdir/${BINARY_NAME}" "$INSTALL_DIR/${BINARY_NAME}"
  else
    echo "sudo 권한으로 ${INSTALL_DIR}에 설치합니다."
    sudo mv "$tmpdir/${BINARY_NAME}" "$INSTALL_DIR/${BINARY_NAME}"
  fi

  chmod +x "$INSTALL_DIR/${BINARY_NAME}"

  echo "설치 완료: $("$INSTALL_DIR/${BINARY_NAME}" --version 2>/dev/null || echo "${BINARY_NAME} (버전 확인 실패, 정상 설치는 됨)")"
  echo "실행: ${BINARY_NAME}"
}

main