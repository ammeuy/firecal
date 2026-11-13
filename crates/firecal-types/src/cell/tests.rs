use super::{CellIndex, GRID_COLUMNS, GRID_ROWS};
use crate::Error;

#[test]
fn north_west_corner_is_index_zero() {
    assert_eq!(CellIndex::new(0, 0).map(CellIndex::index), Ok(0));
}

#[test]
fn second_row_starts_after_one_full_row() {
    assert_eq!(CellIndex::new(1, 0).map(CellIndex::index), Ok(1440));
}

#[test]
fn south_east_corner_is_the_last_index() {
    assert_eq!(
        CellIndex::new(719, 1439).map(CellIndex::index),
        Ok(720 * 1440 - 1)
    );
}

#[test]
fn row_and_column_round_trip() {
    let cell = CellIndex::new(123, 456).unwrap();
    assert_eq!((cell.row(), cell.col()), (123, 456));
}

#[test]
fn row_719_is_inside_and_row_720_is_not() {
    assert!(CellIndex::new(GRID_ROWS - 1, 0).is_ok());
    assert_eq!(CellIndex::new(GRID_ROWS, 0), Err(Error::CellOutsideGrid));
}

#[test]
fn column_1439_is_inside_and_column_1440_is_not() {
    assert!(CellIndex::new(0, GRID_COLUMNS - 1).is_ok());
    assert_eq!(CellIndex::new(0, GRID_COLUMNS), Err(Error::CellOutsideGrid));
}

#[test]
fn every_cell_has_a_distinct_index_in_order() {
    let mut expected = 0;
    for row in 0..GRID_ROWS {
        for col in 0..GRID_COLUMNS {
            assert_eq!(CellIndex::new(row, col).map(CellIndex::index), Ok(expected));
            expected += 1;
        }
    }
}
