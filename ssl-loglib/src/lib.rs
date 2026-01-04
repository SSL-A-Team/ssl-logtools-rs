pub mod protos;
pub mod index;
pub mod log_file_reader;
pub mod raw;

use std::io;
use std::io::Read;
use std::path::Path;
use protobuf::Message;
use chrono::{TimeZone, Utc, DateTime};

use crate::log_file_reader::LogFileReader;
use crate::protos::refbox::ssl_gc_referee_message::Referee;
use crate::protos::vision::messages_robocup_ssl_wrapper_legacy::SSL_WrapperPacket as SSL_WrapperPacket_Legacy;
use crate::protos::vision::messages_robocup_ssl_wrapper::SSL_WrapperPacket;
use crate::protos::vision::messages_robocup_ssl_wrapper_tracked::TrackerWrapperPacket;
use crate::raw::{MessageType, extract_next_raw_message};
use crate::index::IndexMessage;

#[derive(Clone)]
pub enum MessageBody {
    Blank(()),
    Unkown(()),
    Vision2010(SSL_WrapperPacket_Legacy),
    Refbox2013(Referee),
    Vision2014(SSL_WrapperPacket),
    VisionTracker2020(TrackerWrapperPacket),
    Index2021(IndexMessage),
}

#[derive(Clone)]
pub struct LogMessage {
    pub timestamp: DateTime<Utc>,
    pub body: MessageBody,
}

pub fn extract_next_message<R: Read>(reader: &mut R) -> io::Result<LogMessage> {
    let raw_message = extract_next_raw_message(reader)?;
    let timestamp = Utc.timestamp_nanos(raw_message.timestamp);
    let body = match raw_message.message_type {
        MessageType::Blank => MessageBody::Blank(()),
        MessageType::Unkown => MessageBody::Unkown(()),
        MessageType::Vision2010 => MessageBody::Vision2010(SSL_WrapperPacket_Legacy::parse_from_bytes(&raw_message.data)?),
        MessageType::Refbox2013 => MessageBody::Refbox2013(Referee::parse_from_bytes(&raw_message.data)?),
        MessageType::Vision2014 => MessageBody::Vision2014(SSL_WrapperPacket::parse_from_bytes(&raw_message.data)?),
        MessageType::VisionTracker2020 => MessageBody::VisionTracker2020(TrackerWrapperPacket::parse_from_bytes(&raw_message.data)?),
        MessageType::Index2021 => MessageBody::Index2021(IndexMessage::from_bytes(&raw_message.data)?),
    };
    Ok(LogMessage { timestamp, body })
}

pub fn get_all_referee_messages(path: impl AsRef<Path>) -> io::Result<Vec<LogMessage>> {
    let mut reader = LogFileReader::new(path)?;
    let mut ref_messages = Vec::<LogMessage>::new();
    loop {
        match reader.get_next_message() {
            Ok(msg) => {
                if matches!(msg.body, MessageBody::Refbox2013(_)) {
                    ref_messages.push(msg);
                }
            },
            Err(e) => {
                if e.kind() == io::ErrorKind::UnexpectedEof {
                    break;
                } else {
                    return Err(e);
                }
            }
        }
    }
    Ok(ref_messages)
}
