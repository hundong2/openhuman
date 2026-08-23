<!-- rumdl-disable-file MD013 -->

# 03. Memory와 graph orchestration

## 1. 왜 graph인가

단일 agent loop는 process 중단, human approval, sub-agent delegation과 장기 task를 다루기 어렵습니다. OpenHuman은 tinyagents 기반 checkpoint graph를 사용해 node와 transition을 durable state로 표현합니다.

```text
receive
  → plan
  → tool call
  ├─ approval 필요 → parked → resume
  ├─ 실패 → retry / steer / root-cause
  └─ 성공 → verify → complete
```

checkpoint에는 replay에 필요한 최소 state와 version을 남기고 secret·대형 raw tool output을 그대로 저장하지 않는 것이 좋습니다.

실습: [`02_checkpointed_graph.rs`](examples/02_checkpointed_graph.rs)

## 2. Harness entry point

embedding host가 prompt를 turn으로 실행할 때 `openhuman_core::Harness`가 provider, workspace, access, session, MCP와 skill을 typed builder input으로 받습니다.

중요한 계약:

- process당 하나의 Harness
- `workspace_dir`와 `config_path`를 함께 격리
- custom provider도 active app session 요구
- backend call을 위한 실제 또는 stub backend URL 필요
- access tier뿐 아니라 turn origin도 action 권한에 필요
- caller가 tokio worker stack과 blocking thread 수를 적절히 설정

## 3. Memory 구조

Memory Tree는 source item을 chunk·score·summary tree로 정리하고 local SQLite에 저장하며 Obsidian wiki로 mirror할 수 있습니다. vector retrieval만으로 모든 것을 해결한다고 가정하지 않고 사람이 읽고 고칠 수 있는 Markdown representation을 제공합니다.

기억 system을 검증할 때는 다음을 구분합니다.

- source ingestion 성공
- deduplication identity
- stable collection scope
- chunk/retrieval 품질
- summary의 provenance
- stale/삭제 source 반영
- cross-source privacy boundary

## 4. Memory module seam

`tinymemory-api`가 host와 module이 공유하는 contract입니다. OpenHuman의 `memory::api`는 bus를 실제로 건너는 표면만 re-export합니다. module contract와 in-process engine embedding API를 같은 것으로 취급해 export를 넓히면 coupling과 type identity 문제가 생깁니다.

현재 engine migration은 진행 중이므로 `tinymemory-core` direct reference와 module path가 공존합니다. bus에 없는 method를 host에서 성급히 module call로 바꾸면 compile-time failure가 runtime `Unsupported`로 변할 수 있습니다.

## 5. Workflow

tinyflows는 typed node graph를 validate·compile·run합니다. 사용자가 agent의 자동화 제안을 visual canvas에서 검토하고 저장하며 schedule, webhook 또는 channel event로 실행할 수 있습니다.

side effect가 있는 node는 approval gate 뒤에 두고 dry-run capability는 실제 provider·filesystem·network를 사용하지 않는 mock bundle이어야 합니다.

## 6. Agent/tool 경계

tool calling dialect는 catalogue rendering, parsing, result rendering과 transcript replay가 일치해야 합니다. dialect는 tinyagents가 소유하지만 실제 tool 실행 권한, approval, sandbox, timeout과 progress event는 OpenHuman이 소유합니다.

이 경계를 섞으면 parser는 call을 이해하지만 policy를 우회하거나, prompt catalogue와 parser argument 순서가 달라지는 silent failure가 생길 수 있습니다.

다음: [04. Security와 운영](04_security_and_operations.md)
