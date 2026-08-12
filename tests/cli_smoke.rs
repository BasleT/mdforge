use std::process::Command;

fn mdforge() -> Command {
    Command::new(env!("CARGO_BIN_EXE_mdforge"))
}

#[test]
fn help_command_describes_core_commands() {
    let output = mdforge().arg("--help").output().unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("prompt"));
    assert!(stdout.contains("file"));
    assert!(stdout.contains("templates"));
    assert!(stdout.contains("--strict"));
}

#[test]
fn templates_command_lists_built_ins() {
    let output = mdforge().arg("templates").output().unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("default"));
    assert!(stdout.contains("summarize"));
    assert!(stdout.contains("documentation"));
    assert!(stdout.contains("code-docs"));
    assert!(stdout.contains("incident-report"));
}

#[test]
fn missing_file_exits_with_useful_error() {
    let output = mdforge()
        .args(["file", "examples/does-not-exist.txt"])
        .output()
        .unwrap();

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("does not exist"));
}
