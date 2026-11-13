use super::{BASE_YEAR, FIRST_FIRMS_MONTH, MonthIndex};
use crate::Error;

#[test]
fn january_2000_is_index_zero() {
    assert_eq!(MonthIndex::new(2000, 1), Ok(MonthIndex::from_index(0)));
}

#[test]
fn november_2000_is_index_ten() {
    assert_eq!(MonthIndex::new(2000, 11), Ok(FIRST_FIRMS_MONTH));
    assert_eq!(FIRST_FIRMS_MONTH.index(), 10);
}

#[test]
fn january_2001_is_index_twelve() {
    assert_eq!(MonthIndex::new(2001, 1).map(MonthIndex::index), Ok(12));
}

#[test]
fn march_2024_is_index_two_eighty_eight_plus_two() {
    assert_eq!(
        MonthIndex::new(2024, 3).map(MonthIndex::index),
        Ok(24 * 12 + 2)
    );
}

#[test]
fn month_zero_and_thirteen_are_rejected() {
    assert_eq!(MonthIndex::new(2020, 0), Err(Error::MonthOutOfRange));
    assert_eq!(MonthIndex::new(2020, 13), Err(Error::MonthOutOfRange));
}

#[test]
fn year_before_base_is_rejected() {
    assert_eq!(
        MonthIndex::new(BASE_YEAR - 1, 12),
        Err(Error::YearBeforeBase)
    );
}

#[test]
fn year_whose_index_overflows_is_rejected() {
    assert_eq!(MonthIndex::new(u16::MAX, 1), Err(Error::YearTooLate));
}

#[test]
fn april_7461_is_the_last_month_that_fits_and_may_7461_is_not() {
    // (7461 - 2000) * 12 + 3 = 65535, the largest u16.
    assert_eq!(
        MonthIndex::new(7461, 4).map(MonthIndex::index),
        Ok(u16::MAX)
    );
    assert_eq!(MonthIndex::new(7461, 5), Err(Error::YearTooLate));
}

#[test]
fn every_index_round_trips_through_year_and_month() {
    for index in 0..=u16::MAX {
        let month = MonthIndex::from_index(index);
        assert_eq!(
            MonthIndex::new(month.year(), month.month()),
            Ok(month),
            "index {index}"
        );
    }
}

#[test]
fn indexes_order_like_the_calendar() {
    let december_2019 = MonthIndex::new(2019, 12).unwrap();
    let january_2020 = MonthIndex::new(2020, 1).unwrap();
    assert!(december_2019 < january_2020);
    assert_eq!(january_2020.index() - december_2019.index(), 1);
}
