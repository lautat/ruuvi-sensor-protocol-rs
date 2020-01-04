pub(crate) fn u16_from_two_bytes(b1: u8, b2: u8) -> u16 {
    (u16::from(b1) << 8) | u16::from(b2)
}

pub(crate) fn i16_from_two_bytes(b1: u8, b2: u8) -> i16 {
    (i16::from(b1) << 8) | i16::from(b2)
}
