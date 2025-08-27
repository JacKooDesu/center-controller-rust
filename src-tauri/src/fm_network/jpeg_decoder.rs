use std::{io::Read, sync::Arc};

use flate2::read::GzDecoder;

pub struct JPEGDecoder {
    header: JPEGHeader,
    data: Vec<u8>,
    byte_received: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct JPEGHeader {
    label: i32,          // 0 - 3
    id: i32,             // 4 - 7
    length: i32,         // 8 - 11
    offset: i32,         // 12 - 15
    gzip: bool,          // 16
    color_reduction: u8, // 17
}

impl JPEGDecoder {
    pub fn new(header: JPEGHeader) -> Self {
        let data = vec![0; header.length as usize];
        dbg!(header);

        Self {
            data,
            header,
            byte_received: 0,
        }
    }

    pub fn append_data(
        &mut self,
        header: JPEGHeader,
        data: &Vec<u8>,
    ) -> Result<Option<&Vec<u8>>, String> {
        if header.id != self.header.id {
            println!(
                "Warning: JPEG ID mismatch. Expected {}, got {}. Resetting decoder.",
                self.header.id, header.id
            );
            self.header = header;
            self.data = vec![0; header.length as usize];
            self.byte_received = 0;
        }

        let offset = header.offset as usize;
        let data_len = data.len();
        dbg!(data_len);
        dbg!(self.data.len());
        let end_at = offset + data_len;
        if end_at > self.header.length as usize {
            return Err("Data exceeds header length".into());
        }

        self.data[offset..end_at].copy_from_slice(&data[..]);
        self.byte_received += data_len as i32;

        if self.byte_received < self.header.length {
            return Ok(None);
        }

        if self.header.gzip {
            let mut buf = Vec::new();
            let mut decoder = GzDecoder::new(&self.data[..]);
            if let Err(e) = decoder.read_to_end(&mut buf) {
                eprintln!("Error decoding JPEG data: {}", e);
            }

            self.data = buf;
        }
        Ok(Some(&self.data))
    }
}

impl JPEGHeader {
    pub fn new(data: &[u8]) -> Self {
        Self {
            label: i32::from_le_bytes(data[0..4].try_into().unwrap()),
            id: i32::from_le_bytes(data[4..8].try_into().unwrap()),
            length: i32::from_le_bytes(data[8..12].try_into().unwrap()),
            offset: i32::from_le_bytes(data[12..16].try_into().unwrap()),
            gzip: data[16] != 0,
            color_reduction: data[17],
        }
    }
}
