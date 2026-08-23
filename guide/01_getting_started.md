<!-- rumdl-disable-file MD013 -->

# 01. 설치와 첫 실행

## 학습 목표

- native installer와 source build를 구분한다.
- 기본 보안·privacy 설정을 확인한다.
- 전체 desktop build 전에 web UI 또는 좁은 Rust target을 검증한다.

## 사용자 설치

가능하면 운영체제의 native package 경로를 사용합니다.

```bash
# macOS
brew install --cask openhuman

# Debian/Ubuntu: release page에서 architecture에 맞는 .deb를 받은 뒤
sudo apt-get install -y --no-install-recommends ./OpenHuman_*_amd64.deb
```

Windows는 최신 release의 signed MSI를 사용합니다. shell script를 pipe로 바로 실행하는 대체 설치법은 별도 signature로 사전 검증되지 않으므로 native package를 우선하세요.

## 첫 보안 확인

1. Settings의 agent access tier를 확인합니다.
2. `action_dir`가 agent에게 허용할 project root인지 확인합니다.
3. approval gate를 끄기 전에 side effect와 threat model을 검토합니다.
4. Privacy Mode를 쓸 때 모든 workload가 local model을 가리키는지 확인합니다.
5. Gmail·Slack 등 integration의 scope와 자동 fetch를 확인합니다.
6. backup, retention과 Obsidian vault 노출 범위를 결정합니다.

## Source build 요구 사항

- Git과 recursive submodule
- Node.js 24 이상
- pnpm 10.10.0
- repository가 pin한 Rust 1.96.1, `rustfmt`, `clippy`
- CMake, Ninja, ripgrep
- 각 OS의 Tauri desktop build dependency

```bash
git submodule update --init --recursive
pnpm install
```

이 저장소는 여러 vendored Rust crate를 submodule로 사용합니다. 초기화가 빠진 상태에서 dependency 오류를 개별 package 문제로 오인하지 마세요.

## 작은 검증부터 시작

```bash
# web UI만
pnpm dev

# TypeScript
pnpm typecheck

# Rust core
cargo check --manifest-path Cargo.toml

# full desktop
pnpm dev:app
```

macOS Apple Silicon에서 llama.cpp 관련 문제가 있으면 repository 안내에 따라 `GGML_NATIVE=OFF`를 사용합니다.

```bash
GGML_NATIVE=OFF cargo check --manifest-path Cargo.toml
```

## 환경 변수

- root [`.env.example`](../.env.example): Rust core, backend, log와 runtime 설정
- [`app/.env.example`](../app/.env.example): `VITE_*` frontend 설정
- load helper: `source scripts/load-dotenv.sh`

secret이 포함된 `.env`를 commit하지 말고 frontend code에서 `import.meta.env`를 직접 읽지 않습니다. `app/src/utils/config.ts`를 사용합니다.

## 문제 해결

| 증상 | 먼저 볼 것 |
|---|---|
| Rust path dependency 없음 | `git submodule update --init --recursive` |
| Node/pnpm engine 오류 | Node 24+, packageManager pin |
| core RPC 인증 실패 | embedded core의 per-launch bearer, 외부 core token 설정 |
| agent file tool 거부 | access tier, `action_dir`, workspace internal path |
| tool 실행 승인 대기 | approval request와 10분 TTL |
| local-only 기대와 외부 호출 | provider routing, search/integration, Privacy Mode |

다음: [02. 전체 아키텍처](02_architecture.md)
