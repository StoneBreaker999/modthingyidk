use byteorder::ByteOrder;
use std::default::Default;

#[derive(Debug)]
pub struct Sample {
    name: String,
    length: u16,
    finetune: u8,
    volume: u8,
    repeat_point: u16,
    repeat_length: u16,
    //audio: vec![] // TODO!
}

impl Default for Sample {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            length: 0,
            finetune: 0,
            volume: 0,
            repeat_point: 0,
            repeat_length: 0,
        }
    }
}

impl Sample {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_bytes(&self, bytes: &[u8]) -> Self {
        Self {
            name: String::from_utf8_lossy(&bytes[0..22])
                .trim_matches('\0')
                .to_string(),
            length: byteorder::BE::read_u16(&bytes[22..24]),
            finetune: bytes[24],
            volume: bytes[25],

            repeat_point: byteorder::BE::read_u16(&bytes[26..28]),
            repeat_length: byteorder::BE::read_u16(&bytes[28..30]),
        }
    }
}

// TODO!
#[derive(Debug)]
struct Pattern {}

#[derive(Debug)]
pub struct AmigaMod {
    name: String,
    samples: Vec<Sample>,
    length: u8,
    _restart_byte: u8,
    song_positions: Vec<u8>,
    magic: String,
    patterns: Vec<Pattern>, //TODO
}

impl Default for AmigaMod {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            samples: vec![],
            length: 0,
            _restart_byte: 0,
            song_positions: vec![],
            magic: "".to_string(),
            patterns: vec![],
        }
    }
}

impl AmigaMod {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_bytes(&self, bytes: &[u8]) -> Self {
        Self {
            // offset: 0, size: 20
            name: String::from_utf8_lossy(&bytes[0..20])
                .trim_matches('\0')
                .to_string(),

            // offset: 20, size: 30 bytes * 31 entries = 930
            samples: {
                let mut samples: Vec<Sample> = vec![];
                {
                    let mut index = 0;
                    // TODO: make it to include both the descriptors and the audio data itself
                    while index < 930 {
                        samples
                            .push(Sample::new().from_bytes(&bytes[20 + index..(20 + index + 30)]));

                        index += 30;
                    }
                }

                samples
            },

            // offset: 950, size: 1
            length: bytes[950],

            // offset: 951, size: 1
            _restart_byte: bytes[951],

            // offset: 952, size: 128
            song_positions: Vec::from(&bytes[952..1080]),

            // offset: 1080, size: 4
            magic: String::from_utf8_lossy(&bytes[1080..1084]).to_string(),

            // offset: 1084: size: 1024
            patterns: vec![],
            // offset: 2108, size: ..EOF
            //sample_data: bytes[2108..]
        }
    }
}
