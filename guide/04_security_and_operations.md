<!-- rumdl-disable-file MD013 -->

# 04. Security, privacy와 운영

## 1. 권한 결정 순서

```text
tool request
  → command class 분류
  → always-forbidden path 확인
  → access tier / workspace-only / trusted roots
  → approval gate
  → sandbox + Rust path hardening
  → timeout
  → execution + audit/progress
```

알 수 없는 command를 read로 낙관하지 않고 write로 분류하는 fail-closed 정책이 중요합니다.

실습: [`01_security_gate.rs`](examples/01_security_gate.rs)

## 2. Access tier

- `readonly`: mutation 차단
- `supervised`: 위험한 action에 사용자 승인 요구
- `full`: policy 범위 안에서 action 허용

`full`도 system/credential directory와 workspace internal state를 자유롭게 수정한다는 뜻이 아닙니다. `action_dir`, trusted root와 turn origin이 함께 작동합니다.

## 3. Approval과 background task

approval gate는 기본 활성화되고 interactive chat turn을 park합니다. background/cron은 같은 UI 승인 흐름을 사용할 수 없으므로 별도 scope와 side-effect policy가 필요합니다. 사용자가 응답하지 않으면 TTL 후 deny하는 것이 안전합니다.

## 4. Sandbox

- Docker: remote 또는 cron workload 격리
- Local OS jail: Landlock, Seatbelt, AppContainer
- Noop fallback: sandbox가 없더라도 Rust path hardening은 유지

“sandboxed” 표시만 믿지 말고 실제 backend와 fallback을 telemetry에 기록합니다.

## 5. Local-first와 Privacy Mode

local state 암호화와 local inference는 다른 문제입니다. 다음 egress를 각각 확인해야 합니다.

- model provider와 embedding
- managed web search·scraper
- OAuth integration과 auto-fetch
- error reporting·analytics
- messaging channel
- update와 module download

Privacy Mode는 Rust core에서 inference egress를 강제하는 중요한 control이지만 사용자가 명시적으로 연결한 외부 service의 data flow와 retention도 검토해야 합니다.

## 6. Native module supply chain

loadable module은 ABI, manifest, dependency와 SHA-256 digest gate를 통과한 first-party artifact입니다. 그러나 같은 process와 privilege, crash domain을 공유하므로 untrusted code sandbox가 아닙니다.

- registry는 source에 compile된 allowlist 유지
- release digest를 그대로 pin
- OS, architecture와 libc compatibility 확인
- admission 실패를 process lifetime 동안 cache
- strict toolchain match가 실제 release를 거부할 수 있음을 이해
- untrusted extension은 별도 process에서 실행

## 7. 운영 지표

| 영역 | 확인할 지표 |
|---|---|
| Agent | turn completion, retry, halted/root-cause, tool failure |
| Memory | ingestion lag, retrieval hit, stale source, queue depth |
| Workflow | schedule delay, approval wait, replay/resume 성공률 |
| Security | prompt/deny/block, forbidden path, sandbox backend |
| Cost | provider별 token, tool output compression, call cost |
| Reliability | RPC latency, timeout, background service health |

새 flow에는 entry/exit, branch, external call, retry/timeout과 state transition을 grep 가능한 prefix와 correlation field로 기록합니다. secret과 전체 PII는 log에 남기지 않습니다.

## 8. 변경 검증

```bash
pnpm format:check
pnpm typecheck
pnpm test
cargo check --manifest-path Cargo.toml
```

feature gate를 수정하면 enabled build뿐 아니라 `--no-default-features` 방향과 product feature forwarding을 검사합니다. 전체 suite보다 변경 domain의 좁은 target부터 실행하고 영향 범위에 맞춰 넓힙니다.

[가이드 홈](README.md)으로 돌아갑니다.
