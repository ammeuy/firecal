//! Aggregates in, harmonized tiles out: VIIRS mapped to MODIS-equivalent counts, with sigma
//! (see `docs/data-format.md`). The library is pure; only the future binary reads and writes files.
#![forbid(unsafe_code)]
#![deny(clippy::print_stdout, clippy::print_stderr)]
