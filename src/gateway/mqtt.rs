#[derive(serde::Deserialize, Debug)]
struct MqttData {
    #[serde(with = "hex")]
    data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mqtt_data_has_default_traits() {
        crate::testing::type_has_default_traits::<MqttData>();
    }
}
