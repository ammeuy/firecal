use super::month_span;
use firecal_testkit::SeededRandom;
use firecal_types::{Error, MonthIndex};

fn month(year: u16, month: u16) -> MonthIndex {
    MonthIndex::new(year, month).unwrap()
}

#[test]
fn one_month_range_has_span_one() {
    assert_eq!(month_span(month(2020, 5), month(2020, 5)), Ok(1));
}

#[test]
fn a_full_calendar_year_has_span_twelve() {
    assert_eq!(month_span(month(2019, 1), month(2019, 12)), Ok(12));
}

#[test]
fn november_2000_to_december_2001_has_span_fourteen() {
    assert_eq!(month_span(month(2000, 11), month(2001, 12)), Ok(14));
}

#[test]
fn a_last_month_before_the_first_is_empty() {
    assert_eq!(
        month_span(month(2020, 6), month(2020, 5)),
        Err(Error::EmptyMonthRange)
    );
}

#[test]
fn the_longest_possible_range_does_not_fit() {
    let first = MonthIndex::from_index(0);
    let last = MonthIndex::from_index(u16::MAX);
    assert_eq!(month_span(first, last), Err(Error::MonthRangeTooLong));
}

#[test]
fn one_month_short_of_the_longest_range_fits() {
    let first = MonthIndex::from_index(1);
    let last = MonthIndex::from_index(u16::MAX);
    assert_eq!(month_span(first, last), Ok(u16::MAX));
}

#[test]
fn span_of_a_range_built_from_a_length_is_that_length() {
    let seed = 20_261_113;
    let mut random = SeededRandom::new(seed);
    for _ in 0..2_000 {
        let length = u16::try_from(random.next_below(5_000) + 1).unwrap();
        let first_index = u16::try_from(random.next_below(u64::from(u16::MAX - length))).unwrap();
        let first = MonthIndex::from_index(first_index);
        let last = MonthIndex::from_index(first_index + length - 1);
        assert_eq!(month_span(first, last), Ok(length), "seed {seed}");
    }
}
