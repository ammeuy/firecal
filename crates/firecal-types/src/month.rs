//! Months counted from January 2000, the unit of time in every file.

use crate::Error;

/// The first year of the month index.
pub const BASE_YEAR: u16 = 2000;

/// November 2000, the first month in which FIRMS has MODIS detections.
pub const FIRST_FIRMS_MONTH: MonthIndex = MonthIndex(10);

/// A month as `(year - 2000) * 12 + (month - 1)`: January 2000 is 0, November 2000 is 10.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MonthIndex(u16);

impl MonthIndex {
    /// Builds the index of `month` (1 to 12) of `year` (2000 or later).
    ///
    /// # Errors
    /// [`Error::MonthOutOfRange`], [`Error::YearBeforeBase`] or [`Error::YearTooLate`].
    pub fn new(year: u16, month: u16) -> Result<Self, Error> {
        if !(1..=12).contains(&month) {
            return Err(Error::MonthOutOfRange);
        }
        let years_since_base = year.checked_sub(BASE_YEAR).ok_or(Error::YearBeforeBase)?;
        let index = years_since_base
            .checked_mul(12)
            .and_then(|months| months.checked_add(month - 1))
            .ok_or(Error::YearTooLate)?;
        Ok(Self(index))
    }

    /// Wraps a raw index read from a file. Every `u16` is a valid month.
    #[must_use]
    pub const fn from_index(index: u16) -> Self {
        Self(index)
    }

    /// The raw index.
    #[must_use]
    pub const fn index(self) -> u16 {
        self.0
    }

    /// The calendar year, 2000 or later.
    #[must_use]
    pub const fn year(self) -> u16 {
        BASE_YEAR + self.0 / 12
    }

    /// The month of the year, 1 to 12.
    #[must_use]
    pub const fn month(self) -> u16 {
        self.0 % 12 + 1
    }
}

#[cfg(test)]
mod tests;
