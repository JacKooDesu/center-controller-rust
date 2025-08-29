use crate::fm_network::jpeg_decoder::JPEGHeader;

pub enum FMPacket {
    Unknown,
    Heartbeat,
    StringPacket { data: String },
    JPEGPacket { header: JPEGHeader, data: Vec<u8> },
    PlayHistoryPacket { json: String },
}

impl FMPacket {
    pub fn new(raw_data: &[u8]) -> Self {
        if raw_data.len() == 1 {
            return Self::Heartbeat;
        }

        if raw_data.len() <= 2 {
            return Self::Unknown;
        }

        match &raw_data[0] {
            0 => Self::JPEGPacket {
                header: JPEGHeader::new(&raw_data[2..20]),
                data: raw_data[20..].to_vec(),
            },
            1 => Self::decode_string(&raw_data[2..]),
            2 => Self::decode_play_history(&raw_data[2..]),
            _ => Self::Unknown,
        }
    }

    fn decode_string(bytes: &[u8]) -> Self {
        let data = String::from_utf8_lossy(&bytes[0..]).into_owned();
        Self::StringPacket { data }
    }

    fn decode_play_history(bytes: &[u8]) -> Self {
        let json = String::from_utf8_lossy(&bytes[0..]).into_owned();
        Self::PlayHistoryPacket { json }
    }

    pub(crate) fn to_bytes(&self) -> Option<Vec<u8>> {
        match self {
            Self::Unknown => None,
            Self::Heartbeat => Some(vec![1]),
            Self::StringPacket { data } => {
                let mut bytes = vec![];
                bytes.push(1); // meta byte index 0 => string = 1
                bytes.push(0); // target type, not care
                bytes.extend_from_slice(data.as_bytes());
                Some(bytes)
            }
            _ => None,
        }
    }
}
