//! OpenHuman의 command class와 access tier를 단순화한 학습 예제입니다.

#[derive(Clone, Copy, Debug)]
enum AccessTier {
    Readonly,
    Supervised,
    Full,
}

#[derive(Clone, Copy, Debug)]
enum CommandClass {
    Read,
    Write,
    Network,
    Install,
    Destructive,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Decision {
    Allow,
    Prompt,
    Block,
}

fn classify(command: &str) -> CommandClass {
    match command {
        "read_file" | "list_files" => CommandClass::Read,
        "write_file" | "edit_file" => CommandClass::Write,
        "http_get" => CommandClass::Network,
        "install_package" => CommandClass::Install,
        "delete_tree" => CommandClass::Destructive,
        // 모르는 명령을 Read로 취급하면 새 tool이 policy를 우회할 수 있습니다.
        _ => CommandClass::Write,
    }
}

fn gate(tier: AccessTier, class: CommandClass) -> Decision {
    match (tier, class) {
        (AccessTier::Readonly, CommandClass::Read) => Decision::Allow,
        (AccessTier::Readonly, _) => Decision::Block,
        (AccessTier::Supervised, CommandClass::Read) => Decision::Allow,
        (AccessTier::Supervised, _) => Decision::Prompt,
        (AccessTier::Full, CommandClass::Destructive) => Decision::Prompt,
        (AccessTier::Full, _) => Decision::Allow,
    }
}

fn main() {
    let cases = [
        (AccessTier::Readonly, "read_file", Decision::Allow),
        (AccessTier::Readonly, "edit_file", Decision::Block),
        (AccessTier::Supervised, "http_get", Decision::Prompt),
        (AccessTier::Full, "delete_tree", Decision::Prompt),
        (AccessTier::Readonly, "future_tool", Decision::Block),
    ];

    for (tier, command, expected) in cases {
        let decision = gate(tier, classify(command));
        println!("{tier:?} {command:16} -> {decision:?}");
        assert_eq!(decision, expected);
    }
}
