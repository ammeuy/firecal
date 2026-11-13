//! A worktree per agent, so two agents never share a checkout or a `target/` folder.

use crate::owners::Owners;
use crate::shell::{output, run as shell};

/// Creates `../firecal-<owner>-<task>` on a new branch `<owner>/<task>` cut from `origin/main`.
pub(crate) fn run(branch: &str) -> Result<(), String> {
    let (owner, task) = branch
        .split_once('/')
        .ok_or("the branch must look like <owner>/<task>")?;
    let owners = Owners::read_from_repository()?;
    if !owners.has_owner(owner) {
        return Err(format!("`{owner}` is not an owner in OWNERS"));
    }
    if task.is_empty()
        || !task
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return Err(String::from(
            "the task must be lowercase letters, digits and `-`",
        ));
    }
    let root = output("git", &["rev-parse", "--show-toplevel"])?;
    let folder = format!("{root}/../firecal-{owner}-{task}");
    shell("git", &["fetch", "origin", "main"])?;
    shell(
        "git",
        &["worktree", "add", &folder, "-b", branch, "origin/main"],
    )?;
    println!("worktree: cd {folder}");
    Ok(())
}
