use crate::models::MessageType;
use crate::models::SofarMessage;
use bytes::Buf;
use bytes::BytesMut;
use num_traits::FromPrimitive;
use std::io::Cursor;
use tokio_util::codec::Decoder;

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

        let calculated_checksum = &buf
            [1..header_data_length + message_length + footer_data_length - 2]
            .iter()
            .copied()
            .reduce(|a, b| a.wrapping_add(b))
            .unwrap_or(0);

        log::debug!("Calculating checksum: {:?}", calculated_checksum);

        // swallow first byte
        buf.get_u8();
        // swallow message length
        buf.get_u16_le();

        // TODO: handle unknown message type
        let message_type = MessageType::from_u16(buf.get_u16_le()).unwrap();
        log::info!("Got message type: {:?}", message_type);

        // swallow rest of header
        buf.get_u16();
        buf.get_u32();

        let message = match message_type {
            MessageType::Heartbeat => {
                Some(SofarMessage::Heartbeat(bincode::deserialize(&buf).unwrap()))
            }
            MessageType::Data => Some(SofarMessage::Data(bincode::deserialize(&buf).unwrap())),
        };

        buf.advance(message_length);

        let checksum = buf.get_u8();

        if checksum != *calculated_checksum {
            return Err(bincode::Error::new(bincode::ErrorKind::Custom(
                "Invalid checksum".to_string(),
            )));
        }

        // swallow last byte
        buf.get_u8();

        Ok(message)
    }
}

impl Default for SofarCodec {
    fn default() -> Self {
        SofarCodec {}
    }
}
