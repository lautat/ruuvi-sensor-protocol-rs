#[derive(serde::Deserialize, Debug)]
struct MqttData {
    #[serde(deserialize_with = "deserialize_data")]
    data: Vec<u8>,
}

fn deserialize_data<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
    let data: Vec<u8> = hex::serde::deserialize(deserializer)?;

    Ok(data)
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

    #[test]
    fn parse_mqtt_data_switched_packet_order() {
        let data = "
        {
            \"gw_mac\": \"C8:25:2D:8E:9C:2C\",
            \"rssi\": -25,
            \"aoa\": [],
            \"cnt\": \"338\",
            \"data\": \"1BFF990405166455D5C6DE0008FFF403F0AE760F2A8BF41F0C28CBD6020106\",
            \"coords\": \"\"
        }
        ";
        let _mqtt_data: MqttData = serde_json::from_str(data).unwrap();
    }

    #[test]
    fn parse_mqtt_data_no_flags_packet() {
        let data = "
        {
            \"gw_mac\": \"C8:25:2D:8E:9C:2C\",
            \"rssi\": -25,
            \"aoa\": [],
            \"cnt\": \"338\",
            \"data\": \"1BFF990405166455D5C6DE0008FFF403F0AE760F2A8BF41F0C28CBD6\",
            \"coords\": \"\"
        }
        ";
        let _mqtt_data: MqttData = serde_json::from_str(data).unwrap();
    }

    #[test]
    fn parse_mqtt_data_no_manufacturer_data_packet() {
        let data = "
        {
            \"gw_mac\": \"C8:25:2D:8E:9C:2C\",
            \"rssi\": -25,
            \"aoa\": [],
            \"cnt\": \"338\",
            \"data\": \"020106\",
            \"coords\": \"\"
        }
        ";
        let _mqtt_data: MqttData = serde_json::from_str(data).unwrap();
    }

    #[test]
    fn parse_mqtt_data_empty_data() {
        let data = "
        {
            \"gw_mac\": \"C8:25:2D:8E:9C:2C\",
            \"rssi\": -25,
            \"aoa\": [],
            \"cnt\": \"338\",
            \"data\": \"\",
            \"coords\": \"\"
        }
        ";
        let _mqtt_data: MqttData = serde_json::from_str(data).unwrap();
    }

    #[test]
    fn parse_mqtt_data_two_manufacturer_data_packets() {
        let data = "
        {
            \"gw_mac\": \"C8:25:2D:8E:9C:2C\",
            \"rssi\": -25,
            \"aoa\": [],
            \"cnt\": \"338\",
            \"data\": \"1BFF990405166455D5C6DE0008FFF403F0AE760F2A8BF41F0C28CBD6\
                        1BFF990405158A5B05C6810004004403DCAB767A45BDE375CF374E23\",
            \"coords\": \"\"
        }
        ";
        let _mqtt_data: MqttData = serde_json::from_str(data).unwrap();
    }
}
