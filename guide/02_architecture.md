<!-- rumdl-disable-file MD013 -->

# 02. 전체 아키텍처와 RPC 흐름

## 1. 책임 분리

### Rust core

`src/`는 business logic, execution, persistence, domain, RPC와 CLI의 권위 있는 구현입니다. 보안 규칙이나 중요한 상태 전이를 frontend에만 구현하면 다른 host·CLI에서 우회될 수 있습니다.

### Tauri shell

`app/src-tauri/`는 desktop lifecycle, embedded core, native window와 OS integration을 담당하는 얇은 host입니다. core는 sidecar가 아니라 process 내부 tokio task로 실행됩니다.

### React frontend

`app/src/`는 UX, routing, Redux state와 core RPC client를 담당합니다. domain client는 `coreRpcClient`를 통해 Tauri의 HTTP relay command로 요청합니다.

## 2. 요청 흐름

```text
React event
  → domain API service
  → coreRpcClient
  → Tauri relay_http_rpc
  → localhost RPC + per-launch bearer
  → controller schema/handler
  → domain ops
  → store/event bus/tool/harness
  → RpcOutcome
  → Redux/UI
```

core URL과 bearer를 webview에 고정 값으로 노출하지 않고 Tauri command를 통해 전달합니다. CLI·container·cloud 같은 external core는 별도 token contract를 사용합니다.

## 3. Domain 등록

새 기능은 `src/openhuman/<domain>/` 아래에 두고 canonical shape를 따릅니다.

| 파일 | 역할 |
|---|---|
| `mod.rs` | export와 controller registration pair |
| `types.rs` | Serde domain type |
| `store.rs` | persistence |
| `ops.rs` | business logic |
| `schemas.rs` | RPC schema와 handler |
| `tools.rs` | domain 소유 agent tool |
| `bus.rs` | event subscriber |

`src/core/`는 transport만 담당하며 business logic을 넣지 않습니다. controller는 registry에 등록하고 `cli.rs`나 `jsonrpc.rs`에 ad-hoc branch를 추가하지 않습니다.

## 4. Runtime과 compile-time 두 축

- `ServiceSet`: background service와 transport를 선택
- `DomainSet`: runtime에 존재할 domain family를 선택
- Cargo feature: binary에 domain을 compile할지 선택

runtime에서 끈 domain은 controller, tool, store와 subscriber가 함께 사라져야 합니다. compile-time gate는 desktop shell에도 forwarding되어야 실제 shipped product에 들어갑니다. 이 두 축이 어긋나면 controller만 있고 store가 없거나 개발 build에서는 되지만 product에서는 사라지는 문제가 생깁니다.

실습: [`03_domain_registry.rs`](examples/03_domain_registry.rs)

## 5. Event bus

- broadcast: 하나의 event를 여러 subscriber가 수신
- native request/response: process 내부의 typed one-to-one dispatch

domain은 자신의 `bus.rs`를 소유하고 subscriber 이름을 안정적으로 유지합니다. event에 PII나 secret을 넣기 전에 소비자와 log 경로를 점검하세요.

## 6. Frontend 규칙

- shared state는 임의 `localStorage`보다 Redux Toolkit 사용
- 모든 UI text는 `useT()`와 모든 locale의 실제 번역 사용
- Tauri 여부는 `isTauri()` 또는 guarded invoke로 확인
- production `app/src`에서 dynamic import 금지
- child webview에 새 JavaScript injection 금지, Rust-side IPC hook 사용
- analytics에는 user text, ID, filename, credential, error message를 보내지 않음

다음: [03. Memory와 orchestration](03_memory_and_orchestration.md)
