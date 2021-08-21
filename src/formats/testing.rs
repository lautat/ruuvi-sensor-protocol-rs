macro_rules! test_measurement_trait_methods {
    (
        $(
            test $name: ident {
                values: $values: expr,
                expected: {
                    $($method: ident: $result: expr),+,
                },
            }
        )+
    ) => {
        $(
            mod $name {
                use super::*;

                $(
                    #[test]
                    fn $method() {
                        let result = $values;
                        assert_eq!(result.$method(), $result);
                    }
                )+
            }
        )+
    };
}

pub(crate) use test_measurement_trait_methods;
