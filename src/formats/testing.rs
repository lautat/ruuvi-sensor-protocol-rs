#[macro_export]
macro_rules! test_measurement_trait_methods {
    (
        name: $name: ident,
        type_: $type: ty,
        input: $input: expr,
        $($method: ident: $result: expr),+,
    ) => {
        mod $name {
            use super::*;

            $crate::test_measurement_trait_methods! {
                type_: $type,
                input: $input,
                $($method: $result),+,
            }
        }
    };
    (
        type_: $type: ty,
        input: $input: expr,
        $($method: ident: $result: expr),+,
    ) => {
        $crate::test_measurement_trait_methods! {
            values: {
                let value: &[u8] = $input.as_ref();
                <$type>::try_from(value).unwrap()
            },
            $($method: $result),+,
        }
    };
    (
        values: $values: expr,
        $($method: ident: $result: expr),+,
    ) => {
        $(
            $crate::test_measurement_trait_method! {
                name: $method,
                values: $values,
                method: $method,
                expected_value: $result,
            }
        )+
    };
}

#[macro_export]
macro_rules! test_measurement_trait_method {
    (
        name: $name: ident,
        values: $values: expr,
        method: $method: ident,
        expected_value: $result: expr,
    ) => {
        #[test]
        fn $name() {
            let result = $values;
            assert_eq!(result.$method(), $result);
        }
    };
}
