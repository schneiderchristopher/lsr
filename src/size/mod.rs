use std::fmt::{Debug, Display, Write};

pub struct Size(BYTE);

impl Size {
    pub fn new(bytes: BYTE) -> Self {
        Self(bytes)
    }

    pub fn into_exact(&self) -> ExactSize {
        ExactSize(self.0)
    }
}

impl From<BYTE> for Size {
    fn from(value: BYTE) -> Self {
        Self(value)
    }
}

impl From<ExactSize> for Size {
    fn from(value: ExactSize) -> Self {
        Self(value.0)
    }
}

pub trait IntoSize {
    fn into_size(&self) -> Size;
    fn into_exactsize(&self) -> ExactSize;
}

impl IntoSize for u64 {
    fn into_size(&self) -> Size {
        Size::new(*self)
    }
    fn into_exactsize(&self) -> ExactSize {
        ExactSize::new(*self)
    }
}

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
        let bytes = self.0;
        let Some(ByteUnit { size, name }) = units
            // Iterate from the end of the units(largest unit) towards the smallest unit
            .rev()
            // Filter to only include units that are bigger than the size we're trying to format
            .filter(|unit| bytes >= unit.size)
            // Only take the first one that matches, as every one after that is going to be smaller
            .take(1)
            .next() else {
                return f.write_char('0');
            };
        let converted = bytes / size;
        write!(f, "{converted}{name}")
    }
}

pub struct ExactSize(BYTE);

impl ExactSize {
    pub fn new(bytes: BYTE) -> Self {
        Self(bytes)
    }
}

impl From<BYTE> for ExactSize {
    fn from(value: BYTE) -> Self {
        Self(value)
    }
}

impl From<Size> for ExactSize {
    fn from(value: Size) -> Self {
        Self(value.0)
    }
}

impl Display for ExactSize {
    /// Formats the contained size with non-SI units(KiB, real powers of 2),
    /// into every unit it converts into which ends up non-zero
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_units(f, UNITS.iter().copied())
    }
}

impl Debug for ExactSize {
    /// Formats the contained size with SI units, printing every non-zero unit it converts into
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_with_units(f, UNITS_SI.iter().copied())
    }
}

impl<'a> ExactSize {
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

type BYTE = u64;

pub const B: BYTE = 1;
pub const KB: BYTE = B * 1000;
pub const MB: BYTE = KB * 1000;
pub const GB: BYTE = MB * 1000;
pub const TB: BYTE = GB * 1000;
pub const PB: BYTE = TB * 1000;

pub const KIB: BYTE = B * 1_024;
pub const MIB: BYTE = KIB * 1024;
pub const GIB: BYTE = MIB * 1024;
pub const TIB: BYTE = GIB * 1024;
pub const PIB: BYTE = TIB * 1024;

#[derive(Clone, Copy, Debug)]
struct ByteUnit<'a> {
    size: BYTE,
    name: &'a str,
}

impl<'a> ByteUnit<'a> {
    pub const fn new(size: BYTE, name: &'a str) -> Self {
        Self { size, name }
    }
}

impl<'a> From<(BYTE, &'a str)> for ByteUnit<'a> {
    fn from((size, name): (BYTE, &'a str)) -> Self {
        Self { size, name }
    }
}

impl<'a> ByteUnit<'a> {
    const fn from_tuple((size, name): (BYTE, &'a str)) -> Self {
        Self { size, name }
    }
}

const UNITS: [ByteUnit<'static>; 6] = [
    ByteUnit::from_tuple((B, "B")),
    ByteUnit::from_tuple((KIB, "KiB")),
    ByteUnit::from_tuple((MIB, "MiB")),
    ByteUnit::from_tuple((GIB, "GiB")),
    ByteUnit::from_tuple((TIB, "TiB")),
    ByteUnit::from_tuple((PIB, "PiB")),
];
const UNITS_SI: [ByteUnit<'static>; 6] = [
    ByteUnit::from_tuple((B, "B")),
    ByteUnit::from_tuple((KB, "KB")),
    ByteUnit::from_tuple((MB, "MB")),
    ByteUnit::from_tuple((GB, "GB")),
    ByteUnit::from_tuple((TB, "TB")),
    ByteUnit::from_tuple((PB, "PB")),
];

#[cfg(test)]
mod tests {
    use super::*;
    fn size_formatter_test(bytes: BYTE, expected: &str, si: bool) {
        let mut buf = String::new();
        if si {
            write!(&mut buf, "{:?}", bytes.into_size());
        } else {
            write!(&mut buf, "{}", bytes.into_size());
        }
        assert_eq!(buf, expected);
    }

    #[test]
    fn size_formatter_under_1kib() {
        size_formatter_test(495, "495B", false)
    }

    #[test]
    fn size_formatter_exactly_1_kib() {
        size_formatter_test(1024, "1KiB", false)
    }

    #[test]
    fn size_formatter_under_1mib() {
        size_formatter_test(1024 * 512, "512KiB", false)
    }

    #[test]
    fn size_formatter_exactly_1mib() {
        size_formatter_test(1024 * 1024, "1MiB", false)
    }

    #[test]
    fn size_formatter_under_1gib() {
        size_formatter_test(299 * 1024 * 1024, "299MiB", false)
    }

    #[test]
    fn size_formatter_exactly_1gib() {
        size_formatter_test(KIB.pow(3), "1GiB", false)
    }

    #[test]
    fn size_formatter_under_1tib() {
        size_formatter_test(KIB.pow(3) * 128, "128GiB", false)
    }

    #[test]
    fn size_formatter_exactly_1tib() {
        size_formatter_test(KIB.pow(4), "1TiB", false)
    }

    #[test]
    fn size_formatter_under_1pib() {
        size_formatter_test(KIB.pow(4) * 256, "256TiB", false)
    }

    #[test]
    fn size_formatter_exactly_1pib() {
        size_formatter_test(KIB.pow(5), "1PiB", false)
    }

    #[test]
    fn exactsize_formatter_3pib_2gb_3b() {
        let mut buf = String::new();
        write!(&mut buf, "{}", (3 * PIB + 2 * GIB + 3 * B).into_exactsize());
        assert_eq!(buf, "3PiB 2GiB 3B");
    }
}
