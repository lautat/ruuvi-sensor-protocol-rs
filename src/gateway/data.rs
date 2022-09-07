enum Packet<'a> {
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

struct InvalidLength;

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
}
