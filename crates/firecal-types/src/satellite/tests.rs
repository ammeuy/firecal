use super::Satellite;
use crate::Error;

const ALL: [Satellite; 5] = [
    Satellite::Terra,
    Satellite::Aqua,
    Satellite::SuomiNpp,
    Satellite::Noaa20,
    Satellite::Noaa21,
];

#[test]
fn codes_are_the_documented_ones() {
    let codes: [u8; 5] = ALL.map(Satellite::code);
    assert_eq!(codes, [0, 1, 2, 3, 4]);
}

#[test]
fn every_satellite_round_trips_through_its_code() {
    for satellite in ALL {
        assert_eq!(Satellite::from_code(satellite.code()), Ok(satellite));
    }
}

#[test]
fn codes_above_four_are_unknown() {
    assert_eq!(Satellite::from_code(5), Err(Error::UnknownSatellite));
    assert_eq!(Satellite::from_code(u8::MAX), Err(Error::UnknownSatellite));
}

#[test]
fn only_the_last_three_are_viirs() {
    let viirs: [bool; 5] = ALL.map(Satellite::is_viirs);
    assert_eq!(viirs, [false, false, true, true, true]);
}
