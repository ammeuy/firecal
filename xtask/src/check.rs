//! The check every change passes before a pull request.

use crate::shell::{cargo, run as shell};

/// Format, lints, tests and the wasm build. With `full`, also the release profile, which has its own
/// optimizer and must give the same answers.
pub(crate) fn run(full: bool) -> Result<(), String> {
    let cargo = cargo();
    shell(&cargo, &["fmt", "--all", "--", "--check"])?;
    shell(
        &cargo,
        &[
            "clippy",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    shell(&cargo, &["test", "--workspace"])?;
    shell(
        &cargo,
        &[
            "build",
            "--package",
            "firecal-wasm",
            "--target",
            "wasm32-unknown-unknown",
        ],
    )?;
    if full {
        shell(
            &cargo,
            &[
                "clippy",
                "--workspace",
                "--all-targets",
                "--release",
                "--",
                "-D",
                "warnings",
            ],
        )?;
        shell(&cargo, &["test", "--workspace", "--release"])?;
    }
    println!("check: green");
    Ok(())
}
