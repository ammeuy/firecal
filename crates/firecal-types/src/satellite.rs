//! The five satellites whose hotspots we read, kept apart until harmonization.

use crate::Error;

/// A satellite, stored as a `u8` code in the aggregate files.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Satellite {
    /// MODIS on Terra, code 0.
    Terra,
    /// MODIS on Aqua, code 1.
    Aqua,
    /// VIIRS on Suomi NPP, code 2.
    SuomiNpp,
    /// VIIRS on NOAA-20, code 3.
    Noaa20,
    /// VIIRS on NOAA-21, code 4.
    Noaa21,
}

impl Satellite {
    /// Reads the code stored in a file.
    ///
    /// # Errors
    /// [`Error::UnknownSatellite`] for any code above 4.
    pub const fn from_code(code: u8) -> Result<Self, Error> {
        match code {
            0 => Ok(Self::Terra),
            1 => Ok(Self::Aqua),
            2 => Ok(Self::SuomiNpp),
            3 => Ok(Self::Noaa20),
            4 => Ok(Self::Noaa21),
            _ => Err(Error::UnknownSatellite),
        }
    }

    /// The code written to a file.
    #[must_use]
    pub const fn code(self) -> u8 {
        match self {
            Self::Terra => 0,
            Self::Aqua => 1,
            Self::SuomiNpp => 2,
            Self::Noaa20 => 3,
            Self::Noaa21 => 4,
        }
    }

    /// True for the VIIRS satellites, which see smaller fires than MODIS and must be harmonized.
    #[must_use]
    pub const fn is_viirs(self) -> bool {
        matches!(self, Self::SuomiNpp | Self::Noaa20 | Self::Noaa21)
    }
}

#[cfg(test)]
mod tests;
