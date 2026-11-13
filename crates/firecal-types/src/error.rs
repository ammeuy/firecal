//! What can go wrong when a shared type is built from raw numbers.

/// A raw value that does not fit the calendar, the grid or an enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// A month number outside `1..=12`.
    MonthOutOfRange,
    /// A year before 2000, the base of the month index.
    YearBeforeBase,
    /// A year so late that its month index does not fit in 16 bits.
    YearTooLate,
    /// A cell row or column outside the 720 x 1440 grid.
    CellOutsideGrid,
    /// A satellite code that is not one of the five known.
    UnknownSatellite,
    /// A month range whose last month comes before its first.
    EmptyMonthRange,
    /// A month range with more months than fit in 16 bits.
    MonthRangeTooLong,
}
