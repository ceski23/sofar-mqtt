use crate::models::MessageData;
use crate::models::MessageType;
use crate::models::ResponseData;
use crate::models::SofarMessage;
use crate::models::SofarResponseMessage;
use bytes::Buf;
use bytes::BufMut;
use bytes::BytesMut;
use num_traits::FromPrimitive;
use std::io::Cursor;
use tokio_util::codec::Decoder;
use tokio_util::codec::Encoder;

pub struct SofarCodec;

impl Decoder for SofarCodec {
    type Item = SofarMessage;
    type Error = bincode::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        log::debug!("Trying to decode data ({:#?})", buf);
        let header_data_length: usize = 1 + 2 + 2 + 1 + 1 + 4;
        let footer_data_length: usize = 1 + 1;

        if buf.capacity() < header_data_length {
            buf.reserve(header_data_length - buf.capacity())
        }

        if buf.len() < header_data_length {
            log::debug!("Too little data to read header ({:?})", buf.len());
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
            log::debug!("Waiting for more data ({:?})", buf.len());
            return Ok(None);
        }

        let calculated_checksum: u8 =
            calc_checksum(&buf[1..header_data_length + message_length + footer_data_length - 2])
                .unwrap_or(0);

        log::debug!("Calculating checksum: {:?}", calculated_checksum);

        // swallow first byte
        buf.get_u8();
        // swallow message length
        buf.get_u16_le();

        let message_type_bytes = buf.get_u16_le();
        let message_type = MessageType::from_u16(message_type_bytes).ok_or(bincode::Error::new(
            bincode::ErrorKind::Custom(format!("Unknown message type {}", message_type_bytes)),
        ))?;
        log::debug!("Decoded message type: {:?}", message_type);

        let message_number = buf.get_u8();
        let message_number_2 = buf.get_u8();
        let data_logger_sn = buf.get_u32_le();

        let data = match message_type {
            MessageType::Heartbeat => bincode::deserialize(&buf).map(MessageData::Heartbeat),
            MessageType::Data => bincode::deserialize(&buf).map(MessageData::Data),
            MessageType::Hello => bincode::deserialize(&buf).map(MessageData::Hello),
            MessageType::HelloCd => bincode::deserialize(&buf).map(MessageData::HelloCd),
            MessageType::Unknown44 => bincode::deserialize(&buf).map(MessageData::Unknown44),
        }?;

        buf.advance(message_length);

        let checksum = buf.get_u8();

        if checksum != calculated_checksum {
            return Err(bincode::Error::new(bincode::ErrorKind::Custom(
                "Invalid checksum".to_string(),
            )));
        }

        // swallow last byte
        buf.get_u8();

        return Ok(Some(SofarMessage {
            data,
            message_type,
            message_number,
            message_number_2,
            data_logger_sn,
        }));
    }
}

impl Encoder<SofarResponseMessage> for SofarCodec {
    type Error = bincode::Error;

    fn encode(
        &mut self,
        item: SofarResponseMessage,
        buf: &mut BytesMut,
    ) -> Result<(), Self::Error> {
        let data = match item.data {
            ResponseData::ServerResponse(data) => bincode::serialize(&data),
        }?;
        let response_type = get_response_type(item.request_type);

        buf.put_u8(0xa5);
        buf.put_u16_le(u16::try_from(data.len()).unwrap());
        buf.put_u16_le(response_type);
        buf.put_u8(item.request_message_number + 1);
        buf.put_u8(item.request_message_number_2);
        buf.put_u32_le(item.data_logger_sn);
        buf.extend(data);

        let checksum = calc_checksum(&buf[1..]).unwrap();
        buf.put_u8(checksum);
        buf.put_u8(0x15);

        Ok(())
    }
}

impl Default for SofarCodec {
    fn default() -> Self {
        SofarCodec {}
    }
}

fn get_response_type(request_type: MessageType) -> u16 {
    match request_type {
        MessageType::Data => 0x1210,
        MessageType::Heartbeat => 0x1710,
        MessageType::Hello => 0x1110,
        MessageType::HelloCd => 0x1810,
        MessageType::Unknown44 => 0x1310,
    }
}

fn calc_checksum(buf: &[u8]) -> Option<u8> {
    buf.iter().copied().reduce(|a, b| a.wrapping_add(b))
}
