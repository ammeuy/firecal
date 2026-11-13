//! The 0.25 degree grid: 720 rows from north to south, 1440 columns from west to east.

use crate::Error;

/// Columns of the grid: 360 degrees in steps of 0.25.
pub const GRID_COLUMNS: u32 = 1440;

/// Rows of the grid: 180 degrees in steps of 0.25.
pub const GRID_ROWS: u32 = 720;

/// One cell of the grid as `row * 1440 + col`. Row 0 starts at 90 degrees north, column 0 at 180 degrees west.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CellIndex(u32);

impl CellIndex {
    /// Builds the cell at `row` (0 to 719) and `col` (0 to 1439).
    ///
    /// # Errors
    /// [`Error::CellOutsideGrid`] when either is out of range.
    pub const fn new(row: u32, col: u32) -> Result<Self, Error> {
        if row >= GRID_ROWS || col >= GRID_COLUMNS {
            return Err(Error::CellOutsideGrid);
        }
        Ok(Self(row * GRID_COLUMNS + col))
    }

    /// The raw index, `row * 1440 + col`.
    #[must_use]
    pub const fn index(self) -> u32 {
        self.0
    }

    /// The row, 0 at the north pole.
    #[must_use]
    pub const fn row(self) -> u32 {
        self.0 / GRID_COLUMNS
    }

    /// The column, 0 at 180 degrees west.
    #[must_use]
    pub const fn col(self) -> u32 {
        self.0 % GRID_COLUMNS
    }
}

#[cfg(test)]
mod tests;
