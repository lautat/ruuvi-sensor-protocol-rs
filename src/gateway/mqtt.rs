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

    #[test]
    fn parse_mqtt_data_counter() {
        let data = "
        {
            \"gw_mac\": \"C8:25:2D:8E:9C:2C\",
            \"rssi\": -25,
            \"aoa\": [],
            \"cnt\": \"338\",
            \"data\": \"0201061BFF990405166455D5C6DE0008FFF403F0AE760F2A8BF41F0C28CBD6\",
            \"coords\": \"\"
        }
        ";
        let _mqtt_data: MqttData = serde_json::from_str(data).unwrap();
    }

    #[test]
    fn parse_mqtt_data_timestamps() {
        let data = "
        {
            \"gw_mac\": \"C8:25:2D:8E:9C:2C\",
            \"rssi\": -25,
            \"aoa\": [],
            \"ts\": \"1653668027\",
            \"gwts\": \"1653668027\",
            \"data\": \"0201061BFF990405166455D5C6DE0008FFF403F0AE760F2A8BF41F0C28CBD6\",
            \"coords\": \"\"
        }
        ";
        let _mqtt_data: MqttData = serde_json::from_str(data).unwrap();
    }
}
