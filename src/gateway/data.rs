enum Packet<'a> {
    Empty,
    ManufacturerData(&'a [u8]),
    Other(u8, &'a [u8]),
}
