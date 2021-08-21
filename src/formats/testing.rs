#[macro_export]
macro_rules! test_measurement_trait_methods {
    (
        name: $name: ident,
        input: $input: expr,
        $($method: ident: $result: expr),+,
    ) => {
        mod $name {
            use super::*;

            $crate::test_measurement_trait_methods! {
                values: {
                    let value: &[u8] = $input.as_ref();
                    SensorValues::try_from(value).unwrap()
                },
                $($method: $result),+,
            }
        }
    };
    (
        values: $values: expr,
        $($method: ident: $result: expr),+,
    ) => {
        $(
            #[test]
            fn $method() {
                let result = $values;
                assert_eq!(result.$method(), $result);
            }
        )+
    };
}

#[macro_export]
macro_rules! test_parser {
    (
        name: $name: ident,
        input: $input: expr,
        result: $result: expr,
    ) => {
        #[test]
        fn $name() {
            let input: &[u8] = $input.as_ref();
            let result = SensorValues::try_from(input);
            assert_eq!(result, $result);
        }
    };
}
