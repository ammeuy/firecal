//! Who owns which files, read from the `OWNERS` file at the root of the repository.
//!
//! Format: one line per owner, `name: path path ...`; a path ending in `/` is a directory, anything else is
//! one file. The owner `shared` lists paths every branch may touch. `#` starts a comment.

use crate::shell::output;
use std::fs;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Owners {
    entries: Vec<(String, Vec<String>)>,
}

impl Owners {
    /// Reads `OWNERS` from the root of the current repository.
    pub(crate) fn read_from_repository() -> Result<Self, String> {
        let root = output("git", &["rev-parse", "--show-toplevel"])?;
        let path = format!("{root}/OWNERS");
        let text =
            fs::read_to_string(&path).map_err(|error| format!("cannot read {path}: {error}"))?;
        Self::parse(&text)
    }

    /// Parses the text of an `OWNERS` file.
    pub(crate) fn parse(text: &str) -> Result<Self, String> {
        let mut entries = Vec::new();
        for (number, raw) in text.lines().enumerate() {
            let line = raw.split('#').next().unwrap_or_default().trim();
            if line.is_empty() {
                continue;
            }
            let (name, paths) = line
                .split_once(':')
                .ok_or_else(|| format!("OWNERS line {}: expected `name: paths`", number + 1))?;
            let paths: Vec<String> = paths.split_whitespace().map(str::to_owned).collect();
            if name.trim().is_empty() || paths.is_empty() {
                return Err(format!(
                    "OWNERS line {}: an owner needs a name and paths",
                    number + 1
                ));
            }
            entries.push((name.trim().to_owned(), paths));
        }
        Ok(Self { entries })
    }

    /// True when `name` appears as an owner (the `shared` entry is not an owner).
    pub(crate) fn has_owner(&self, name: &str) -> bool {
        name != "shared" && self.entries.iter().any(|(owner, _)| owner == name)
    }

    /// True when `owner` may edit `file`, by owning it or because it is shared.
    pub(crate) fn may_edit(&self, owner: &str, file: &str) -> bool {
        self.entries
            .iter()
            .filter(|(name, _)| name == owner || name == "shared")
            .flat_map(|(_, paths)| paths)
            .any(|path| {
                if path.ends_with('/') {
                    file.starts_with(path.as_str())
                } else {
                    file == path
                }
            })
    }
}

#[cfg(test)]
mod tests;
