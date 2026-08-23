//! 중단 후 재개 가능한 agent graph의 핵심을 단순화한 예제입니다.

#[derive(Clone, Copy, Debug, PartialEq)]
enum Node {
    Plan,
    Tool,
    Verify,
    Complete,
}

#[derive(Debug)]
struct RunState {
    version: u32,
    node: Node,
    attempts: u8,
    tool_available: bool,
}

fn step(state: &mut RunState) -> Result<(), &'static str> {
    state.node = match state.node {
        Node::Plan => Node::Tool,
        Node::Tool if state.tool_available => Node::Verify,
        Node::Tool => {
            state.attempts += 1;
            return Err("tool temporarily unavailable; checkpoint and resume");
        }
        Node::Verify => Node::Complete,
        Node::Complete => Node::Complete,
    };
    Ok(())
}

fn checkpoint(state: &RunState) -> String {
    // 실제 시스템에서는 atomic durable store와 schema migration을 사용합니다.
    format!("v{}:{:?}:{}", state.version, state.node, state.attempts)
}

fn main() {
    let mut state = RunState {
        version: 1,
        node: Node::Plan,
        attempts: 0,
        tool_available: false,
    };

    step(&mut state).expect("planning should succeed");
    assert_eq!(state.node, Node::Tool);

    let error = step(&mut state).expect_err("tool should fail once");
    println!("halted: {error}");
    println!("checkpoint: {}", checkpoint(&state));

    // 외부 조건이 회복된 뒤 같은 node에서 재개합니다.
    state.tool_available = true;
    while state.node != Node::Complete {
        step(&mut state).expect("resumed graph should complete");
        println!("checkpoint: {}", checkpoint(&state));
    }
}
