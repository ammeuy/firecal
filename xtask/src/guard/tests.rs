use super::foreign_files;
use crate::owners::Owners;

fn owners() -> Owners {
    Owners::parse("shared: requests/\nbiggie: Cargo.toml\ndre: crates/firecal-ingest/\n").unwrap()
}

#[test]
fn a_branch_that_stays_in_its_directory_has_no_foreign_files() {
    let files = [
        "crates/firecal-ingest/src/lib.rs",
        "requests/dre-to-biggie-x.md",
    ];
    assert_eq!(
        foreign_files(&owners(), "dre/parse-csv", &files),
        Ok(vec![])
    );
}

#[test]
fn a_lead_only_file_edited_by_someone_else_is_foreign() {
    let files = ["crates/firecal-ingest/src/lib.rs", "Cargo.toml"];
    assert_eq!(
        foreign_files(&owners(), "dre/parse-csv", &files),
        Ok(vec!["Cargo.toml"])
    );
}

#[test]
fn a_branch_without_a_slash_is_rejected() {
    assert!(foreign_files(&owners(), "parse-csv", &[]).is_err());
}

#[test]
fn a_branch_with_an_unknown_owner_is_rejected() {
    assert!(foreign_files(&owners(), "nobody/task", &[]).is_err());
}

#[test]
fn the_shared_prefix_cannot_be_used_as_a_branch_owner() {
    assert!(foreign_files(&owners(), "shared/task", &[]).is_err());
}

#[test]
fn an_empty_diff_is_fine() {
    assert_eq!(foreign_files(&owners(), "dre/nothing", &[]), Ok(vec![]));
}
