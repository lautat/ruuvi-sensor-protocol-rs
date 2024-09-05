use alloc::borrow::Cow;

use serde::{
    de::{Error, Unexpected},
    Deserialize,
};

use crate::{
    gateway::data::{IterPackets, Packet},
    SensorValues,
};

/// MQTT Message payload sent by Ruuvi Gateway
///
/// At the moment, only the `data` field is parsed from the payload although it may contain other
/// fields.
#[expect(clippy::module_name_repetitions)]
#[derive(serde::Deserialize, Debug)]
pub struct MqttData {
    /// RuuviTag sensor values parsed from the message payload
    #[serde(deserialize_with = "deserialize_data")]
    pub data: SensorValues,
}

fn deserialize_data<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<SensorValues, D::Error> {
    let encoded: Cow<'_, str> = Deserialize::deserialize(deserializer)?;
    let decoded = hex::decode(encoded.as_ref()).map_err(|_| {
        D::Error::invalid_value(
            Unexpected::Str(&encoded),
            &"a hex-encoded Bluetooth advertisement data",
        )
    })?;

    let mut packets = IterPackets::new(&decoded);
    let manufacturer_data = packets.try_fold(None, |result, packet| match (result, packet) {
        (None, Ok(Packet::ManufacturerData(id, data))) => Ok(Some((id, data))),
        (_, Err(err)) => Err(err),
        (result, _) => Ok(result),
    });

    if let Ok(Some((id, data))) = manufacturer_data {
        SensorValues::from_manufacturer_specific_data(id, data).map_err(|_| {
            D::Error::invalid_value(
                Unexpected::Str(&encoded),
                &"an advertisement containing a valid Ruuvi manufacturer data packet",
            )
        })
    } else {
        let error = D::Error::invalid_value(
            Unexpected::Str(&encoded),
            &"a valid advertisement containing a manufacturer data packet",
        );
        Err(error)
    }
}

#[expect(clippy::unwrap_used)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{MacAddress, MeasurementSequenceNumber};

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
        let mqtt_data: MqttData = serde_json::from_str(data).unwrap();

        assert_eq!(
            mqtt_data.data.mac_address(),
            Some([0xF4, 0x1F, 0x0C, 0x28, 0xCB, 0xD6])
        );
        assert_eq!(mqtt_data.data.measurement_sequence_number(), Some(10891));
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
        let mqtt_data: MqttData = serde_json::from_str(data).unwrap();

        assert_eq!(
            mqtt_data.data.mac_address(),
            Some([0xF4, 0x1F, 0x0C, 0x28, 0xCB, 0xD6])
        );
        assert_eq!(mqtt_data.data.measurement_sequence_number(), Some(10891));
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
        let mqtt_data: MqttData = serde_json::from_str(data).unwrap();

        assert_eq!(
            mqtt_data.data.mac_address(),
            Some([0xF4, 0x1F, 0x0C, 0x28, 0xCB, 0xD6])
        );
        assert_eq!(mqtt_data.data.measurement_sequence_number(), Some(10891));
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
        let mqtt_data: MqttData = serde_json::from_str(data).unwrap();

        assert_eq!(
            mqtt_data.data.mac_address(),
            Some([0xF4, 0x1F, 0x0C, 0x28, 0xCB, 0xD6])
        );
        assert_eq!(mqtt_data.data.measurement_sequence_number(), Some(10891));
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

        mqtt_data.unwrap_err();
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

        mqtt_data.unwrap_err();
    }

    #[test]
    fn parse_mqtt_data_two_manufacturer_data_packets() {
        let data = "\
        {
            \"gw_mac\": \"C8:25:2D:8E:9C:2C\",
            \"rssi\": -25,
            \"aoa\": [],
            \"cnt\": \"338\",
            \"data\": \"1BFF990405158A5B05C6810004004403DCAB767A45BDE375CF374E23\
                        1BFF990405166455D5C6DE0008FFF403F0AE760F2A8BF41F0C28CBD6\",
            \"coords\": \"\"
        }\
        ";
        let mqtt_data: MqttData = serde_json::from_str(data).unwrap();

        assert_eq!(
            mqtt_data.data.mac_address(),
            Some([0xE3, 0x75, 0xCF, 0x37, 0x4E, 0x23])
        );
        assert_eq!(mqtt_data.data.measurement_sequence_number(), Some(17853));
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

        mqtt_data.unwrap_err();
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

        mqtt_data.unwrap_err();
    }
}
