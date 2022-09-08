use serde::de::{Error, Unexpected};

use crate::{gateway::data::{IterPackets, Packet}, SensorValues};

#[derive(serde::Deserialize, Debug)]
struct MqttData {
    #[serde(deserialize_with = "deserialize_data")]
    data: SensorValues,
}

fn deserialize_data<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<SensorValues, D::Error> {
    let data: Vec<u8> = hex::serde::deserialize(deserializer)?;
    let mut packets = IterPackets::new(&data);
    let manufacturer_data = packets.try_fold(None, |result, packet| match (result, packet) {
        (None, Ok(Packet::ManufacturerData(id, data))) => Ok(Some((id, data))),
        (_, Err(err)) => Err(err),
        (result, _) => Ok(result),
    });

    if let Ok(Some((id, data))) = manufacturer_data {
        if let Ok(values) = SensorValues::from_manufacturer_specific_data(id, data) {
            Ok(values)
        } else {
            let error = D::Error::invalid_value(
                Unexpected::Bytes(&data),
                &"an advertisement containing a valid Ruuvi manufacturer data packet",
            );
            Err(error)
        }
    } else {
        let error = D::Error::invalid_value(
            Unexpected::Bytes(&data),
            &"a valid advertisement containing a manufacturer data packet",
        );
        Err(error)
    }
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
        let data = "\
        {
            \"gw_mac\": \"C8:25:2D:8E:9C:2C\",
            \"rssi\": -25,
            \"aoa\": [],
            \"cnt\": \"338\",
            \"data\": \"0201061BFF990405166455D5C6DE0008FFF403F0AE760F2A8BF41F0C28CBD6\",
            \"coords\": \"\"
        }\
        ";
        let _mqtt_data: MqttData = serde_json::from_str(data).unwrap();
    }

    #[test]
    fn parse_mqtt_data_timestamps() {
        let data = "\
        {
            \"gw_mac\": \"C8:25:2D:8E:9C:2C\",
            \"rssi\": -25,
            \"aoa\": [],
            \"ts\": \"1653668027\",
            \"gwts\": \"1653668027\",
            \"data\": \"0201061BFF990405166455D5C6DE0008FFF403F0AE760F2A8BF41F0C28CBD6\",
            \"coords\": \"\"
        }\
        ";
        let _mqtt_data: MqttData = serde_json::from_str(data).unwrap();
    }

    #[test]
    fn parse_mqtt_data_switched_packet_order() {
        let data = "\
        {
            \"gw_mac\": \"C8:25:2D:8E:9C:2C\",
            \"rssi\": -25,
            \"aoa\": [],
            \"cnt\": \"338\",
            \"data\": \"1BFF990405166455D5C6DE0008FFF403F0AE760F2A8BF41F0C28CBD6020106\",
            \"coords\": \"\"
        }\
        ";
        let _mqtt_data: MqttData = serde_json::from_str(data).unwrap();
    }

    #[test]
    fn parse_mqtt_data_no_flags_packet() {
        let data = "\
        {
            \"gw_mac\": \"C8:25:2D:8E:9C:2C\",
            \"rssi\": -25,
            \"aoa\": [],
            \"cnt\": \"338\",
            \"data\": \"1BFF990405166455D5C6DE0008FFF403F0AE760F2A8BF41F0C28CBD6\",
            \"coords\": \"\"
        }\
        ";
        let _mqtt_data: MqttData = serde_json::from_str(data).unwrap();
    }

    #[test]
    fn parse_mqtt_data_no_manufacturer_data_packet() {
        let data = "\
        {
            \"gw_mac\": \"C8:25:2D:8E:9C:2C\",
            \"rssi\": -25,
            \"aoa\": [],
            \"cnt\": \"338\",
            \"data\": \"020106\",
            \"coords\": \"\"
        }\
        ";
        let mqtt_data: Result<MqttData, _> = serde_json::from_str(data);

        assert!(mqtt_data.is_err());
    }

    #[test]
    fn parse_mqtt_data_empty_data() {
        let data = "\
        {
            \"gw_mac\": \"C8:25:2D:8E:9C:2C\",
            \"rssi\": -25,
            \"aoa\": [],
            \"cnt\": \"338\",
            \"data\": \"\",
            \"coords\": \"\"
        }\
        ";
        let mqtt_data: Result<MqttData, _> = serde_json::from_str(data);

        assert!(mqtt_data.is_err());
    }

    #[test]
    fn parse_mqtt_data_two_manufacturer_data_packets() {
        let data = "\
        {
            \"gw_mac\": \"C8:25:2D:8E:9C:2C\",
            \"rssi\": -25,
            \"aoa\": [],
            \"cnt\": \"338\",
            \"data\": \"1BFF990405166455D5C6DE0008FFF403F0AE760F2A8BF41F0C28CBD6\
                        1BFF990405158A5B05C6810004004403DCAB767A45BDE375CF374E23\",
            \"coords\": \"\"
        }\
        ";
        let _mqtt_data: MqttData = serde_json::from_str(data).unwrap();
    }

    #[test]
    fn parse_mqtt_data_invalid_manufacturer() {
        let data = "\
        {
            \"gw_mac\": \"C8:25:2D:8E:9C:2C\",
            \"rssi\": -25,
            \"aoa\": [],
            \"cnt\": \"338\",
            \"data\": \"0201061BFF9A0405166455D5C6DE0008FFF403F0AE760F2A8BF41F0C28CBD6\",
            \"coords\": \"\"
        }\
        ";
        let mqtt_data: Result<MqttData, _> = serde_json::from_str(data);

        assert!(mqtt_data.is_err());
    }

    #[test]
    fn parse_mqtt_data_invalid_data() {
        let data = "\
        {
            \"gw_mac\": \"C8:25:2D:8E:9C:2C\",
            \"rssi\": -25,
            \"aoa\": [],
            \"cnt\": \"338\",
            \"data\": \"0201061BFF990405166455D5C6DE0008FFF403F0AE760F2A8BF41F0C28CB\",
            \"coords\": \"\"
        }\
        ";
        let mqtt_data: Result<MqttData, _> = serde_json::from_str(data);

        assert!(mqtt_data.is_err());
    }
}
