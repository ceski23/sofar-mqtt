use crate::messages::IncomingMessageData;
use crate::messages::OutgoingMessageData;
use crate::messages::SofarMessage;
use crate::messages::SofarMessageType;
use anyhow::anyhow;
use bytes::Buf;
use bytes::BufMut;
use bytes::BytesMut;
use num_traits::FromPrimitive;
use std::io::Cursor;
use tokio_util::codec::Decoder;
use tokio_util::codec::Encoder;
use tracing::debug;

#[derive(Default)]
pub struct SofarCodec;

impl Decoder for SofarCodec {
    type Item = SofarMessage<IncomingMessageData>;
    type Error = anyhow::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> anyhow::Result<Option<Self::Item>, Self::Error> {
        debug!("Trying to decode data ({:#?})", buf);
        let header_data_length: usize = 1 + 2 + 2 + 1 + 1 + 4;
        let footer_data_length: usize = 1 + 1;

        if buf.capacity() < header_data_length {
            buf.reserve(header_data_length - buf.capacity())
        }

        if buf.len() < header_data_length {
            debug!("Too little data to read header ({:?})", buf.len());
            return Ok(None);
        }

        let mut peeker = Cursor::new(&buf[..header_data_length]);

        // swallow first byte
        peeker.get_u8();

        let message_length = peeker.get_u16_le() as usize;

        if buf.capacity() < header_data_length + message_length + footer_data_length {
            buf.reserve(header_data_length + message_length + footer_data_length - buf.capacity())
        }

        if buf.len() < (header_data_length + message_length + footer_data_length) {
            debug!("Waiting for more data ({:?})", buf.len());
            return Ok(None);
        }

        let calculated_checksum: u8 =
            calc_checksum(&buf[1..header_data_length + message_length + footer_data_length - 2])
                .unwrap_or(0);

        debug!("Calculating checksum: {:?}", calculated_checksum);

        // swallow first byte
        buf.get_u8();
        // swallow message length
        buf.get_u16_le();

        let message_type_bytes = buf.get_u16_le();
        let message_type = SofarMessageType::from_u16(message_type_bytes)
            .ok_or(anyhow!("Unknown message type {message_type_bytes}"))?;
        debug!("Decoded message type: {:?}", message_type);

        let message_number = buf.get_u8();
        let message_number_2 = buf.get_u8();
        let data_logger_sn = buf.get_u32_le();

        let data = match message_type {
            SofarMessageType::Heartbeat => {
                bincode::deserialize(buf).map(IncomingMessageData::Heartbeat)
            }
            SofarMessageType::Data => bincode::deserialize(buf).map(IncomingMessageData::Data),
            SofarMessageType::Hello => bincode::deserialize(buf).map(IncomingMessageData::Hello),
            SofarMessageType::HelloCd => {
                bincode::deserialize(buf).map(IncomingMessageData::HelloCd)
            }
            SofarMessageType::Unknown44 => {
                bincode::deserialize(buf).map(IncomingMessageData::Unknown44)
            }
        }?;

        debug!("Decoded payload: {:?}", data);

        buf.advance(message_length);

        let checksum = buf.get_u8();

        if checksum != calculated_checksum {
            return Err(anyhow!("Invalid checksum {checksum}"));
        }

        // swallow last byte
        buf.get_u8();

        Ok(Some(SofarMessage {
            data,
            message_type,
            message_number,
            message_number_2,
            data_logger_sn,
        }))
    }
}

impl Encoder<SofarMessage<OutgoingMessageData>> for SofarCodec {
    type Error = bincode::Error;

    fn encode(
        &mut self,
        item: SofarMessage<OutgoingMessageData>,
        buf: &mut BytesMut,
    ) -> anyhow::Result<(), Self::Error> {
        debug!("Payload to encode: {:?}", item);

        let response_type = match item.message_type {
            SofarMessageType::Data => 0x1210,
            SofarMessageType::Heartbeat => 0x1710,
            SofarMessageType::Hello => 0x1110,
            SofarMessageType::HelloCd => 0x1810,
            SofarMessageType::Unknown44 => 0x1310,
        };
        let data = match item.data {
            OutgoingMessageData::ServerResponse(data) => bincode::serialize(&data),
        }?;

        buf.put_u8(0xa5);
        buf.put_u16_le(u16::try_from(data.len()).unwrap());
        buf.put_u16_le(response_type);
        buf.put_u8(item.message_number);
        buf.put_u8(item.message_number_2);
        buf.put_u32_le(item.data_logger_sn);
        buf.extend(data);

        let checksum = calc_checksum(&buf[1..]).unwrap();
        buf.put_u8(checksum);
        buf.put_u8(0x15);

        Ok(())
    }
}

fn calc_checksum(buf: &[u8]) -> Option<u8> {
    buf.iter().copied().reduce(|a, b| a.wrapping_add(b))
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use tokio_util::codec::{Decoder, Encoder};

    use crate::{
        codec::SofarCodec,
        messages::{IncomingMessageData, SofarMessage},
    };

    #[test]
    fn hello_message() {
        let mut message_bytes = BytesMut::from_iter(vec![
            165, 86, 0, 16, 65, 3, 4, 79, 172, 254, 103, 2, 71, 125, 14, 0, 127, 0, 0, 0, 0, 0, 0,
            0, 5, 60, 120, 2, 25, 1, 76, 83, 87, 51, 95, 49, 52, 95, 70, 70, 70, 70, 95, 49, 46,
            48, 46, 51, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 234,
            231, 44, 60, 22, 49, 48, 46, 48, 46, 48, 46, 54, 52, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1,
            39, 127, 21,
        ]);
        let expected_response_bytes = BytesMut::from_iter(vec![
            165, 10, 0, 16, 17, 4, 4, 79, 172, 254, 103, 2, 1, 140, 39, 103, 100, 120, 0, 0, 0,
            140, 21,
        ]);

        let mut codec = SofarCodec::default();
        let message = codec.decode(&mut message_bytes).unwrap().unwrap();

        assert!(matches!(message.data, IncomingMessageData::Hello { .. }));

        let mut response_bytes = BytesMut::new();
        let response_message = SofarMessage::from_incoming_message(&message, 1684481932);
        codec.encode(response_message, &mut response_bytes).unwrap();

        assert_eq!(response_bytes, expected_response_bytes);
    }

    #[test]
    fn hello_end_message() {
        let mut message_bytes = BytesMut::from_iter(vec![
            165, 60, 0, 16, 72, 9, 13, 79, 172, 254, 103, 1, 194, 133, 14, 0, 139, 0, 0, 0, 110,
            170, 88, 100, 1, 5, 44, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 197, 21,
        ]);
        let expected_response_bytes = BytesMut::from_iter(vec![
            165, 10, 0, 16, 24, 10, 13, 79, 172, 254, 103, 1, 1, 48, 48, 103, 100, 120, 0, 0, 0,
            78, 21,
        ]);

        let mut codec = SofarCodec::default();
        let message = codec.decode(&mut message_bytes).unwrap().unwrap();

        assert!(matches!(message.data, IncomingMessageData::HelloCd { .. }));

        let mut response_bytes = BytesMut::new();
        let response_message = SofarMessage::from_incoming_message(&message, 1684484144);
        codec.encode(response_message, &mut response_bytes).unwrap();

        assert_eq!(response_bytes, expected_response_bytes);
    }

    #[test]
    fn data_message() {
        let mut message_bytes = BytesMut::from_iter(vec![
            165, 151, 0, 16, 66, 4, 5, 79, 172, 254, 103, 1, 1, 39, 72, 125, 14, 0, 128, 0, 0, 0,
            69, 170, 88, 100, 1, 0, 40, 13, 0, 0, 83, 70, 52, 69, 83, 48, 48, 51, 77, 52, 67, 48,
            53, 56, 32, 32, 104, 1, 122, 11, 213, 2, 12, 0, 0, 0, 9, 0, 10, 0, 9, 0, 195, 8, 216,
            8, 201, 8, 135, 19, 54, 1, 0, 0, 69, 0, 0, 0, 174, 126, 0, 0, 220, 24, 0, 0, 2, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 86, 50, 56, 48, 86, 49, 48, 48, 21, 0, 4, 24,
            100, 11, 193, 2, 60, 0, 1, 0, 40, 5, 87, 6, 33, 5, 7, 0, 0, 0, 0, 0, 6, 0, 226, 3, 227,
            3, 227, 3, 86, 50, 56, 48, 86, 50, 56, 48, 23, 5, 19, 9, 36, 49, 37, 0, 0, 0, 96, 21,
        ]);
        let expected_response_bytes = BytesMut::from_iter(vec![
            165, 10, 0, 16, 18, 5, 5, 79, 172, 254, 103, 1, 1, 141, 39, 103, 100, 120, 0, 0, 0,
            143, 21,
        ]);

        let mut codec = SofarCodec::default();
        let message = codec.decode(&mut message_bytes).unwrap().unwrap();

        assert!(matches!(message.data, IncomingMessageData::Data { .. }));

        let mut response_bytes = BytesMut::new();
        let response_message = SofarMessage::from_incoming_message(&message, 1684481933);
        codec.encode(response_message, &mut response_bytes).unwrap();

        assert_eq!(response_bytes, expected_response_bytes);
    }

    #[test]
    fn heartbeat_message() {
        let mut message_bytes = BytesMut::from_iter(vec![
            165, 1, 0, 16, 71, 31, 32, 79, 172, 254, 103, 0, 247, 21,
        ]);
        let expected_response_bytes = BytesMut::from_iter(vec![
            165, 10, 0, 16, 23, 32, 32, 79, 172, 254, 103, 0, 1, 141, 39, 103, 100, 120, 0, 0, 0,
            201, 21,
        ]);

        let mut codec = SofarCodec::default();
        let message = codec.decode(&mut message_bytes).unwrap().unwrap();

        assert!(matches!(
            message.data,
            IncomingMessageData::Heartbeat { .. }
        ));

        let mut response_bytes = BytesMut::new();
        let response_message = SofarMessage::from_incoming_message(&message, 1684481933);
        codec.encode(response_message, &mut response_bytes).unwrap();

        assert_eq!(response_bytes, expected_response_bytes);
    }
}
