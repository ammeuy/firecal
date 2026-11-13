//! How many months a range covers.

use firecal_types::{Error, MonthIndex};

/// The number of months from `first` to `last`, both included.
///
/// # Errors
/// [`Error::EmptyMonthRange`] when `last` is before `first`, [`Error::MonthRangeTooLong`] when the
/// count does not fit in 16 bits.
pub fn month_span(first: MonthIndex, last: MonthIndex) -> Result<u16, Error> {
    last.index()
        .checked_sub(first.index())
        .ok_or(Error::EmptyMonthRange)?
        .checked_add(1)
        .ok_or(Error::MonthRangeTooLong)
}

#[cfg(test)]
mod tests;
