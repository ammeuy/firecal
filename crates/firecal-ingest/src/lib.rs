//! FIRMS hotspot files in, per-satellite cell-month aggregates out (`.fca`, see `docs/data-format.md`).
//!
//! The library is pure: it parses bytes and aggregates. Only the future `firecal-ingest` binary opens files.
#![forbid(unsafe_code)]
#![deny(clippy::print_stdout, clippy::print_stderr)]
