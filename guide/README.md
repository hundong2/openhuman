<!-- rumdl-disable-file MD013 -->

# OpenHuman 한국어 학습 가이드

[원본 README](../README.md) · [한국어 README](../docs/README.ko.md)

OpenHuman은 단순 chat UI가 아니라 Rust core, durable agent graph, local memory, security policy, React/Tauri desktop shell과 다양한 integration을 결합한 대규모 agent harness입니다. 이 가이드는 installer로 사용하는 단계에서 시작해 저장소를 읽고 안전하게 확장하는 수준까지 이어집니다.

## 학습 순서

1. [01. 설치와 첫 실행](01_getting_started.md)
2. [02. 전체 아키텍처와 RPC 흐름](02_architecture.md)
3. [03. Memory와 graph orchestration](03_memory_and_orchestration.md)
4. [04. Security, privacy와 운영](04_security_and_operations.md)
5. [Rust 실습 예제](examples/README.md)

## 단계별 목표

| 단계 | 결과 | 대형 build 필요 |
|---|---|---|
| 사용자 | native installer와 Privacy Mode의 의미를 이해 | 아니요 |
| 입문 개발자 | Rust core와 React/Tauri의 책임 경계를 추적 | 아니요 |
| 중급 개발자 | checkpoint graph, memory, domain registration을 설명 | 선택 |
| 고급 개발자 | access tier, approval, sandbox, feature gate와 운영 검증 설계 | 선택 |

## 전체 지도

```text
React UI (app/src)
  │ coreRpcClient
  ↓
Tauri relay + in-process Rust core
  │ JSON-RPC / typed controller
  ↓
Domain operations ──→ SQLite / local encrypted state
  │
  ├─ agent harness ──→ tinyagents checkpoint graph
  ├─ memory ─────────→ TinyMemory/TinyCortex + Obsidian mirror
  ├─ flows ──────────→ tinyflows durable automation
  ├─ integrations ───→ OAuth / MCP / channels
  └─ security ───────→ policy → approval → sandbox/path hardening
```

## 중요한 전제

- `main`은 early beta로 빠르게 변하므로 현재 source, `AGENTS.md`, lockfile과 release note를 최종 기준으로 삼습니다.
- product claim과 실제 build feature는 다를 수 있습니다. `scripts/ci/product-features.txt`와 desktop shell forwarding을 확인하세요.
- local-first는 자동으로 “외부 전송 없음”을 뜻하지 않습니다. 선택한 model provider, search, integration과 telemetry 설정을 확인해야 합니다.
- OpenHuman code는 GPL-3.0-only이며 외부 model·service·connector에는 별도 license와 약관이 적용될 수 있습니다.
- repository 내부 `AGENTS.md`는 architecture와 안전 불변식을 매우 상세히 설명합니다. 실제 code를 수정하기 전에 전체를 읽어야 합니다.

## 저장소를 읽는 추천 순서

```text
README.md / INSTALL.md
  → AGENTS.md
  → src/core/runtime/builder.rs
  → src/core/all.rs
  → src/embed/ + src/harness.rs 계열
  → src/openhuman/agent/
  → src/openhuman/memory/
  → app/src/services/coreRpcClient 관련 경로
  → tests/와 gitbooks/developing/
```

등록 표면부터 보면 domain implementation이 실제 runtime과 UI에 어떻게 노출되는지 이해하기 쉽습니다.
