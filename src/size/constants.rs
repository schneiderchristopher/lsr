use super::ByteUnit;

pub type BYTE = u64;

pub const MAX_SIZE_LEN: usize = 7;

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

pub const UNITS: [ByteUnit<'static>; 6] = [
    ByteUnit::from_tuple((B, "b")),
    ByteUnit::from_tuple((KIB, "k")),
    ByteUnit::from_tuple((MIB, "m")),
    ByteUnit::from_tuple((GIB, "g")),
    ByteUnit::from_tuple((TIB, "t")),
    ByteUnit::from_tuple((PIB, "p")),
];
pub const UNITS_SI: [ByteUnit<'static>; 6] = [
    ByteUnit::from_tuple((B, "B")),
    ByteUnit::from_tuple((KB, "K")),
    ByteUnit::from_tuple((MB, "M")),
    ByteUnit::from_tuple((GB, "G")),
    ByteUnit::from_tuple((TB, "T")),
    ByteUnit::from_tuple((PB, "P")),
];
