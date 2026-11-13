use super::SeededRandom;

#[test]
fn seed_one_gives_the_values_an_independent_python_implementation_gave() {
    let mut random = SeededRandom::new(1);
    assert_eq!(random.next_u64(), 0x47e4_ce4b_896c_dd1d);
    assert_eq!(random.next_u64(), 0xabcf_a6a8_e079_651d);
    assert_eq!(random.next_u64(), 0xb9d1_0d8f_eb73_1f57);
}

#[test]
fn the_same_seed_repeats_the_same_sequence() {
    let mut first = SeededRandom::new(42);
    let mut second = SeededRandom::new(42);
    for _ in 0..100 {
        assert_eq!(first.next_u64(), second.next_u64());
    }
}

#[test]
fn different_seeds_give_different_sequences() {
    assert_ne!(
        SeededRandom::new(1).next_u64(),
        SeededRandom::new(2).next_u64()
    );
}

#[test]
fn a_zero_seed_does_not_get_stuck_at_zero() {
    let mut random = SeededRandom::new(0);
    assert_ne!(random.next_u64(), 0);
    assert_ne!(random.next_u64(), 0);
}

#[test]
fn next_below_stays_below_its_bound() {
    let mut random = SeededRandom::new(7);
    for _ in 0..10_000 {
        assert!(random.next_below(10) < 10);
    }
}

#[test]
fn next_below_with_bound_zero_returns_zero() {
    assert_eq!(SeededRandom::new(7).next_below(0), 0);
}

#[test]
fn next_below_reaches_every_value_of_a_small_bound() {
    let mut random = SeededRandom::new(3);
    let mut seen = [false; 4];
    for _ in 0..1_000 {
        let value = usize::try_from(random.next_below(4)).unwrap();
        seen[value] = true;
    }
    assert_eq!(seen, [true; 4]);
}
