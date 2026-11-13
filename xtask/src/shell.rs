//! Running other programs and saying what was run.

use std::process::Command;

/// The `cargo` that started this task, so the pinned toolchain is the one used.
pub(crate) fn cargo() -> String {
    std::env::var("CARGO").unwrap_or_else(|_| String::from("cargo"))
}

/// Runs `program` with `arguments`, echoing the command, and fails if it does.
pub(crate) fn run(program: &str, arguments: &[&str]) -> Result<(), String> {
    let line = format!("{program} {}", arguments.join(" "));
    println!("$ {line}");
    let status = Command::new(program)
        .args(arguments)
        .status()
        .map_err(|error| format!("cannot start `{program}`: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("`{line}` failed: {status}"))
    }
}

/// Runs `program` and returns what it printed, trimmed, failing if it does.
pub(crate) fn output(program: &str, arguments: &[&str]) -> Result<String, String> {
    let line = format!("{program} {}", arguments.join(" "));
    let result = Command::new(program)
        .args(arguments)
        .output()
        .map_err(|error| format!("cannot start `{program}`: {error}"))?;
    if result.status.success() {
        Ok(String::from_utf8_lossy(&result.stdout).trim().to_owned())
    } else {
        Err(format!(
            "`{line}` failed: {}",
            String::from_utf8_lossy(&result.stderr).trim()
        ))
    }
}
