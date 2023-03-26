use std::fmt::{Debug, Display};
mod constants;
mod convenience;
pub use constants::*;
pub use convenience::*;

use crate::ZERO_SIZE;

pub struct Size(BYTE);

impl Display for Size {
    /// Formats the contained size with non-SI units(KiB, real powers of 2), into the first unit it
    /// converts to as non-zero
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_units(f, UNITS.iter().copied())
    }
}

impl Debug for Size {
    /// Formats the contained size with SI units, into the first unit it converts to as non-zero
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_units(f, UNITS_SI.iter().copied())
    }
}

impl<'a> Size {
    fn fmt_with_units<I>(&self, f: &mut std::fmt::Formatter<'_>, units: I) -> std::fmt::Result
    where
        I: Iterator<Item = ByteUnit<'a>> + std::iter::DoubleEndedIterator,
    {
        let width = f.width().unwrap_or_default().max(1) - 1;
        let bytes = self.0;
        let Some(ByteUnit { size, name }) = units
            // Iterate from the end of the units(largest unit) towards the smallest unit
            .rev()
            // Filter to only include units that are bigger than the size we're trying to format
            .filter(|unit| bytes >= unit.size)
            // Only take the first one that matches, as every one after that is going to be smaller
            .take(1)
            .next() else {
                return match f.width() {
                Some(width) => write!(f, "{:>width$}", ZERO_SIZE, width=width+1),
                    None => write!(f, "{}", ZERO_SIZE )
                }
            };
        let converted = bytes / size;
        write!(f, "{converted:width$}{name}", width = width)
    }
}

pub struct LongSize(BYTE);

impl Display for LongSize {
    /// Formats the contained size with non-SI units(KiB, real powers of 2),
    /// into every unit it converts into which ends up non-zero
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_units(f, UNITS.iter().copied())
    }
}

impl Debug for LongSize {
    /// Formats the contained size with SI units, printing every non-zero unit it converts into
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_units(f, UNITS_SI.iter().copied())
    }
}

impl<'a> LongSize {
    fn fmt_with_units<I>(&self, f: &mut std::fmt::Formatter<'_>, units: I) -> std::fmt::Result
    where
        I: Iterator<Item = ByteUnit<'a>> + std::iter::DoubleEndedIterator,
    {
        let mut bytes = self.0;
        let mut units = units
            // Iterate from the end of the units(largest unit) towards the smallest unit
            .rev();
        // Write first unit without a space at the beginning
        let first_unit = 'firstunit: {
            while let Some(unit) = units.next() {
                if bytes >= unit.size {
                    break 'firstunit Some(unit);
                }
            }
            None
        };
        let Some(ByteUnit { size, name }) = first_unit else {
        return write!(f, "0")};
        let converted = bytes / size;
        bytes -= converted * size;
        write!(f, "{converted}{name}")?;
        for ByteUnit { size, name } in units {
            // Filter to only include units that are bigger than the size we're trying to format
            if bytes < size {
                continue;
            }
            let converted = bytes / size;
            bytes -= converted * size;
            write!(f, " {converted}{name}")?
        }

        Ok(())
    }
}

pub struct DecimalSize(BYTE);

impl Display for DecimalSize {
    /// Formats the contained size with non-SI units(KiB, real powers of 2), into the first unit it
    /// converts to as non-zero
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_units(f, UNITS.iter().copied())
    }
}

impl Debug for DecimalSize {
    /// Formats the contained size with SI units, into the first unit it converts to as non-zero
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_units(f, UNITS_SI.iter().copied())
    }
}

impl<'a> DecimalSize {
    fn fmt_with_units<I>(&self, f: &mut std::fmt::Formatter<'_>, units: I) -> std::fmt::Result
    where
        I: Iterator<Item = ByteUnit<'a>> + std::iter::DoubleEndedIterator,
    {
        let bytes = self.0;
        let Some(ByteUnit { size, name }) = units
            // Iterate from the end of the units(largest unit) towards the smallest unit
            .rev()
            // Filter to only include units that are bigger than the size we're trying to format
            .filter(|unit| bytes >= unit.size)
            // Only take the first one that matches, as every one after that is going to be smaller
            .take(1)
            .next() else {
                return match f.width() {
                Some(width) => write!(f, "{:>width$}", ZERO_SIZE, width=width+1),
                    None => write!(f, "{}", ZERO_SIZE )
                }
            };
        let converted = bytes as f32 / size as f32;

        let round = converted.fract() < 0.1;
        // FIXME: Rather incomplete alignment implementation, but good enough for what we're doing
        match (f.width(), f.align()) {
            (Some(width), Some(_)) => {
                if round {
                    write!(f, "{converted:>width$.0}{name}", width = width)
                } else {
                    write!(f, "{converted:>width$.1}{name}", width = width)
                }
            }
            (Some(width), None) => {
                if round {
                    write!(f, "{converted:width$.0}{name}", width = width)
                } else {
                    write!(f, "{converted:width$}{name}", width = width)
                }
            }
            _ => Ok(()),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ByteUnit<'a> {
    size: BYTE,
    name: &'a str,
}

impl<'a> ByteUnit<'a> {
    pub const fn new(size: BYTE, name: &'a str) -> Self {
        Self { size, name }
    }
}

mod tests;
