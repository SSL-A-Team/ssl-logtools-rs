use std::io::{Error, ErrorKind, Read};
use std::mem::size_of;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum MessageType {
    Blank = 0,
    Unkown = 1,
    Vision2010 = 2,
    Refbox2013 = 3,
    Vision2014 = 4,
    VisionTracker2020 = 5,
    Index2021 = 6,
}

impl TryFrom<i32> for MessageType {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MessageType::Blank),
            1 => Ok(MessageType::Unkown),
            2 => Ok(MessageType::Vision2010),
            3 => Ok(MessageType::Refbox2013),
            4 => Ok(MessageType::Vision2014),
            5 => Ok(MessageType::VisionTracker2020),
            6 => Ok(MessageType::Index2021),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Unsupported message type: {}", value),
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RawMessage {
    pub timestamp: i64,
    pub message_type: MessageType,
    pub data: Vec<u8>,
}

pub fn extract_next_raw_message<R: Read>(reader: &mut R) -> Result<RawMessage, std::io::Error> {
    let mut timestamp_buffer = [0; size_of::<i64>()];
    reader.read_exact(&mut timestamp_buffer)?;
    let timestamp = i64::from_be_bytes(timestamp_buffer);
    let mut message_type_buffer = [0; size_of::<i32>()];
    reader.read_exact(&mut message_type_buffer)?;
    let message_type = i32::from_be_bytes(message_type_buffer);
    let mut message_size_buffer = [0; size_of::<i32>()];
    reader.read_exact(&mut message_size_buffer)?;
    let message_size = i32::from_be_bytes(message_size_buffer);
    let mut data_buffer = vec![0; message_size as usize];
    reader.read_exact(&mut data_buffer)?;
    Ok(RawMessage {
        timestamp,
        message_type: MessageType::try_from(message_type)?,
        data: data_buffer,
    })
}
