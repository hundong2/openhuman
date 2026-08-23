//! compile-time availability와 runtime DomainSet을 함께 적용하는 예제입니다.

#[derive(Clone, Copy, Debug, PartialEq)]
enum Domain {
    Agent,
    Memory,
    Flows,
    Web3,
}

struct DomainSpec {
    domain: Domain,
    controller: &'static str,
    compiled_in: bool,
}

fn runtime_enabled(domain: Domain) -> bool {
    matches!(domain, Domain::Agent | Domain::Memory | Domain::Flows)
}

fn main() {
    let specs = [
        DomainSpec { domain: Domain::Agent, controller: "agent.run", compiled_in: true },
        DomainSpec { domain: Domain::Memory, controller: "memory.search", compiled_in: true },
        DomainSpec { domain: Domain::Flows, controller: "flows.run", compiled_in: true },
        DomainSpec { domain: Domain::Web3, controller: "wallet.transfer", compiled_in: false },
    ];

    let registered: Vec<_> = specs
        .iter()
        .filter(|spec| spec.compiled_in && runtime_enabled(spec.domain))
        .map(|spec| spec.controller)
        .collect();

    println!("registered controllers: {registered:?}");
    assert!(registered.contains(&"agent.run"));
    assert!(registered.contains(&"memory.search"));
    assert!(registered.contains(&"flows.run"));
    assert!(!registered.contains(&"wallet.transfer"));
}
