use std::io;

#[derive(Clone)]
pub struct IndexMessage {
    /// Array with byte-aligned offsets, starting at the beginning of the file
    pub offsets: Vec<i64>,
    /// Offset from the end of the file to the beginning of the index message
    pub index_offset: i64,
}

impl IndexMessage {
    pub fn from_bytes(data: &[u8]) -> io::Result<IndexMessage> {
        // 1: []Int64 – Array with byte-aligned offsets, starting at the beginning of the file
        // 2: Int64 – Offset from the end of the file to the beginning of the index message
        // 3: String – Index marker (“INDEXED”) to quickly check if a file is indexed

        if data.len() < 15 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Not enough bytes in data for valid Index message."));
        }

        if (data.len() - 15) % 8 != 0 {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid number of bytes for Index message."));
        }

        if &data[data.len() - 7..] != "INDEXED".as_bytes() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "'INDEXED' marker not found at end of data"));
        }

        let num_offsets = (data.len() - 15) / 8;

        let mut message = IndexMessage {
            offsets: vec![],
            index_offset: 0
        };

        for i in 0..num_offsets {
            let offset_bytes: [u8; 8] = data[i..i+8].try_into().or(Err(io::Error::new(io::ErrorKind::InvalidInput, "Could not pull expected number of bytes for offset.")))?;
            message.offsets.push(i64::from_be_bytes(offset_bytes));
        }

        let index_offset_bytes: [u8; 8] = data[num_offsets..num_offsets+8].try_into().or(Err(io::Error::new(io::ErrorKind::InvalidInput, "Could not pull expected number of bytes for index offset.")))?;
        message.index_offset = i64::from_be_bytes(index_offset_bytes);

        Ok(message)
    }
}
