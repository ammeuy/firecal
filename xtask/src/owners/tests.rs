use super::Owners;

const SAMPLE: &str = "
# a comment
shared: requests/
biggie: Cargo.toml crates/firecal-types/   # trailing comment
dre: crates/firecal-ingest/
";

fn owners() -> Owners {
    Owners::parse(SAMPLE).unwrap()
}

#[test]
fn an_owner_may_edit_inside_their_directory() {
    assert!(owners().may_edit("dre", "crates/firecal-ingest/src/lib.rs"));
}

#[test]
fn an_owner_may_not_edit_another_owners_directory() {
    assert!(!owners().may_edit("dre", "crates/firecal-types/src/lib.rs"));
}

#[test]
fn a_file_entry_matches_only_that_exact_file() {
    assert!(owners().may_edit("biggie", "Cargo.toml"));
    assert!(!owners().may_edit("biggie", "Cargo.toml.bak"));
    assert!(!owners().may_edit("dre", "Cargo.toml"));
}

#[test]
fn a_directory_entry_does_not_match_a_sibling_with_the_same_prefix() {
    assert!(!owners().may_edit("dre", "crates/firecal-ingest-extra/src/lib.rs"));
}

#[test]
fn shared_paths_are_open_to_every_owner() {
    assert!(owners().may_edit("dre", "requests/dre-to-biggie-clock.md"));
    assert!(owners().may_edit("biggie", "requests/dre-to-biggie-clock.md"));
}

#[test]
fn shared_is_not_an_owner() {
    assert!(!owners().has_owner("shared"));
    assert!(owners().has_owner("dre"));
}

#[test]
fn an_unknown_name_may_edit_only_shared_paths() {
    assert!(!owners().may_edit("nobody", "Cargo.toml"));
    assert!(owners().may_edit("nobody", "requests/x.md"));
}

#[test]
fn a_line_without_a_colon_is_an_error() {
    assert!(Owners::parse("dre crates/").is_err());
}

#[test]
fn an_owner_without_paths_is_an_error() {
    assert!(Owners::parse("dre:").is_err());
}
