use super::{INVALID, fc_month_index, fc_month_span};

#[test]
fn month_index_of_november_2000_is_ten() {
    assert_eq!(fc_month_index(2000, 11), 10);
}

#[test]
fn month_index_rejects_a_bad_month_without_trapping() {
    assert_eq!(fc_month_index(2020, 13), INVALID);
}

#[test]
fn month_index_rejects_numbers_wider_than_sixteen_bits() {
    assert_eq!(fc_month_index(70_000, 1), INVALID);
    assert_eq!(fc_month_index(2020, 65_537), INVALID);
}

#[test]
fn month_span_counts_both_ends() {
    assert_eq!(fc_month_span(10, 21), 12);
}

#[test]
fn month_span_of_an_empty_range_is_invalid() {
    assert_eq!(fc_month_span(21, 10), INVALID);
}

#[test]
fn month_span_rejects_numbers_wider_than_sixteen_bits() {
    assert_eq!(fc_month_span(0, 70_000), INVALID);
}
