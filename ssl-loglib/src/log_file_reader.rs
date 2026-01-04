use crate::{LogMessage, extract_next_message};
use std::fs::File;
use std::io::Read;
use std::io::Seek;
use std::io;
use std::path::Path;

pub struct LogFileReader {
    file: File,
}

impl LogFileReader {
    pub fn new(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = File::open(path)?;
        let mut reader = LogFileReader {
            file,
        };
        reader.verify_log_preamble()?;
        Ok(reader)
    }

    pub fn get_next_message(&mut self) -> io::Result<LogMessage> {
        extract_next_message(&mut self.file)
    }

    pub fn is_indexed(&mut self) -> io::Result<bool> {
        const EXPECTED_MARKER: &[u8] = b"INDEXED";
        let position_cache = self.file.stream_position()?;
        self.file.seek(io::SeekFrom::End(-(EXPECTED_MARKER.len() as i64)))?;
        let mut marker_buffer = [0; EXPECTED_MARKER.len()];
        self.file.read_exact(&mut marker_buffer)?;
        self.file.seek(io::SeekFrom::Start(position_cache))?;
        Ok(marker_buffer == EXPECTED_MARKER)
    }

    fn verify_log_preamble(&mut self) -> io::Result<()> {
        const EXPECTED_PREAMBLE: &[u8] = b"SSL_LOG_FILE";
        let mut preamble_buffer = [0; EXPECTED_PREAMBLE.len()];
        self.file.read_exact(&mut preamble_buffer)?;
        if preamble_buffer != EXPECTED_PREAMBLE {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Log file preamble does not match expected value",
            ));
        }
        const EXPECTED_VERSION: i32 = 1;
        let mut version_buffer = [0; 4];
        self.file.read_exact(&mut version_buffer)?;
        let version = i32::from_be_bytes(version_buffer);
        if version != EXPECTED_VERSION {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unsupported log format version: {}", version),
            ));
        }
        Ok(())
    }
}

impl Iterator for LogFileReader {
    type Item = LogMessage;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.get_next_message();
        match res {
            Ok(m) => Some(m),
            Err(e) => {
                if e.kind() != io::ErrorKind::UnexpectedEof {
                    eprintln!("{}", e);
                }
                None
            }
        }
    }
}
