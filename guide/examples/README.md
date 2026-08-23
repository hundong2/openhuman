<!-- rumdl-disable-file MD013 -->

# Rust 실습 예제

세 예제는 OpenHuman crate를 직접 link하지 않는 standalone Rust 2021 프로그램입니다. 대형 dependency build 없이 architecture의 핵심 불변식을 연습하기 위한 toy implementation이며 실제 security·checkpoint·registry 구현을 대체하지 않습니다.

## 실행

```bash
rustc --edition 2021 01_security_gate.rs -o security_gate
./security_gate

rustc --edition 2021 02_checkpointed_graph.rs -o checkpointed_graph
./checkpointed_graph

rustc --edition 2021 03_domain_registry.rs -o domain_registry
./domain_registry
```

Windows PowerShell에서는 생성된 binary에 `.exe`를 붙여 실행합니다.

## 학습 목표

### 1. Security gate

command class와 access tier를 분리하고 unknown command를 write로 fail-closed 분류합니다. “full”이어도 destructive action은 별도 approval을 요구하도록 확인합니다.

### 2. Checkpointed graph

node 실행 후 state를 checkpoint하고 실패한 tool node에서 resume합니다. 실제 구현은 durable store와 versioned schema가 필요하지만 state machine의 핵심을 작은 code로 볼 수 있습니다.

### 3. Domain registry

compile-time availability와 runtime `DomainSet`을 모두 통과한 controller만 등록합니다. runtime toggle 하나가 controller·tool·store에 일관되게 적용되어야 하는 이유를 보여줍니다.

## 확장 과제

- security 예제에 trusted root와 always-forbidden path 추가
- checkpoint를 text file에 atomic write하고 schema version 검증
- domain registry에 store/subscriber accounting guard 추가
- 각 예제를 `#[test]` 기반 unit test로 변환
