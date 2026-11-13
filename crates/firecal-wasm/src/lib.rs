//! The browser build. Every export is a plain `extern "C"` function over numbers or linear-memory
//! buffers, described in `wasm/firecal.d.ts`; there is no `wasm-bindgen`.
//!
//! Status convention: a function that can fail returns [`INVALID`] (`u32::MAX`) instead of trapping.
#![expect(
    unsafe_code,
    reason = "the only unsafe here is the no_mangle attribute that exports a function"
)]
#![deny(clippy::print_stdout, clippy::print_stderr)]

use firecal_query::month_span;
use firecal_types::MonthIndex;

/// Returned by an export that could not do what was asked.
pub const INVALID: u32 = u32::MAX;

/// The month index of `month` (1 to 12) of `year`, or [`INVALID`].
#[unsafe(no_mangle)]
pub extern "C" fn fc_month_index(year: u32, month: u32) -> u32 {
    let (Ok(year), Ok(month)) = (u16::try_from(year), u16::try_from(month)) else {
        return INVALID;
    };
    MonthIndex::new(year, month).map_or(INVALID, |index| u32::from(index.index()))
}

/// The number of months from `first` to `last` inclusive, or [`INVALID`].
#[unsafe(no_mangle)]
pub extern "C" fn fc_month_span(first: u32, last: u32) -> u32 {
    let (Ok(first), Ok(last)) = (u16::try_from(first), u16::try_from(last)) else {
        return INVALID;
    };
    month_span(MonthIndex::from_index(first), MonthIndex::from_index(last))
        .map_or(INVALID, u32::from)
}

#[cfg(test)]
mod tests;
