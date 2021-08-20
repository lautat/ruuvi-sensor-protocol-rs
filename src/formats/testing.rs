#[macro_export]
macro_rules! test_conversion_methods {
    (
        type_: $type: ty,
        input: $input: expr,
        $($method: ident: $result: expr),+,
    ) => {
        $(
            $crate::test_conversion_method! {
                name: $method,
                type_: $type,
                input: $input,
                method: $method,
                expected_value: $result,
            }
        )+
    };
    (
        name: $name: ident,
        type_: $type: ty,
        input: $input: expr,
        $($method: ident: $result: expr),+,
    ) => {
        mod $name {
            use super::*;

            $(
                $crate::test_conversion_method! {
                    name: $method,
                    type_: $type,
                    input: $input,
                    method: $method,
                    expected_value: $result,
                }
            )+
        }
    };
}

#[macro_export]
macro_rules! test_conversion_method {
    (
        name: $name: ident,
        type_: $type: ty,
        input: $input: expr,
        method: $method: ident,
        expected_value: $result: expr,
    ) => {
        #[test]
        fn $name() {
            let value: &[u8] = $input.as_ref();
            let result = <$type>::try_from(value).unwrap();
            assert_eq!(result.$method(), $result);
        }
    };
}
