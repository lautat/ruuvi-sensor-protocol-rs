#[macro_export]
macro_rules! test_measurement_trait_methods {
    (
        name: $name: ident,
        values: $values: expr,
        $($method: ident: $result: expr),+,
    ) => {
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
    };
}
