pub struct IterPackets<'a> {
    data: &'a [u8],
    index: usize,
}

impl<'a> IterPackets<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        let index = 0;
        Self { data, index }
    }
}

impl<'a> Iterator for IterPackets<'a> {
    type Item = Result<Packet<'a>, InvalidPacket>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let len = self.data[self.index];
            let start = self.index + 1;
            let end = start + usize::from(len);

            if end <= self.data.len() {
                self.index = end;
                Some(Ok(Packet::from(&self.data[start..end])))
            } else {
                self.index = self.data.len();
                Some(Err(InvalidPacket))
            }
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Packet<'a> {
    Empty,
    ManufacturerData(&'a [u8]),
    Other(u8, &'a [u8]),
}

impl<'a> From<&'a [u8]> for Packet<'a> {
    fn from(data: &'a [u8]) -> Self {
        match data {
            [] => Self::Empty,
            [0xFF, data @ ..] => Self::ManufacturerData(data),
            [typ, data @ ..] => Self::Other(*typ, data),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct InvalidPacket;

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_packet_from_slice {
        (
            $(
                test $name: ident {
                    input: $input: expr,
                    result: $result: expr,
                }
            )+
        ) => {
            mod packet_from_slice {
                use super::*;

                $(
                    #[test]
                    fn $name() {
                        let data = $input;
                        let packet = Packet::from(data.as_slice());
                        assert_eq!(packet, $result);
                    }
                )+
            }
        };
    }

    macro_rules! test_iter_packets {
        (
            $(
                test $name: ident {
                    input: $input: expr,
                    results: [
                        $($result: expr,)+
                    ],
                }
            )+
        ) => {
            mod iter_packets {
                use super::*;

                $(
                    #[test]
                    fn $name() {
                        let data = $input;
                        let mut iter = IterPackets::new(data.as_slice());

                        $(
                            assert_eq!(iter.next(), $result);
                        )+
                    }
                )+
            }
        }
    }

    test_packet_from_slice! {
        test empty_slice {
            input: [],
            result: Packet::Empty,
        }

        test manufacturer_data_1 {
            input: [0xFF],
            result: Packet::ManufacturerData(&[]),
        }

        test manufacturer_data_2 {
            input: [0xFF, 0x00, 0x01],
            result: Packet::ManufacturerData(&[0x00, 0x01]),
        }

        test manufacturer_data_3 {
            input: [0xFF, 0xAB, 0xCD],
            result: Packet::ManufacturerData(&[0xAB, 0xCD]),
        }

        test other_1 {
            input: [0x01],
            result: Packet::Other(0x01, &[]),
        }

        test other_2 {
            input: [0x02, 0x03, 0x04],
            result: Packet::Other(0x02, &[0x03, 0x04]),
        }

        test other_3 {
            input: [0x01, 0xCD, 0xEF, 0x00],
            result: Packet::Other(0x01, &[0xCD, 0xEF, 0x00]),
        }
    }

    test_iter_packets! {
        test empty {
            input: [],
            results: [
                None,
            ],
        }

        test one_item {
            input: [0x02, 0x00, 0x01],
            results: [
                Some(Ok(Packet::Other(0x00, &[0x01]))),
                None,
            ],
        }

        test multiple_items {
            input: [0x03, 0xFF, 0xAB, 0xCD, 0x00, 0x02, 0x01, 0xFF],
            results: [
                Some(Ok(Packet::ManufacturerData(&[0xAB, 0xCD]))),
                Some(Ok(Packet::Empty)),
                Some(Ok(Packet::Other(0x01, &[0xFF]))),
                None,
            ],
        }

        test invalid_end {
            input: [0x03, 0xFF, 0xAB, 0xCD, 0x00, 0x03, 0x01, 0xFF],
            results: [
                Some(Ok(Packet::ManufacturerData(&[0xAB, 0xCD]))),
                Some(Ok(Packet::Empty)),
                Some(Err(InvalidPacket)),
                None,
            ],
        }
    }
}
