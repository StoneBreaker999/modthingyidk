use std::default::Default;

use byteorder::ByteOrder;

use crate::offsets::*;

#[derive(Debug)]
pub struct ModFile {
    song_name: String,
    sample_info: Vec<SampleInfo>, // TODO: make this better
    song_length: u8,
    restart_byte: u8, // TODO: find better name
    song_position: u8,
    identifier: String,
    // this is at the end of the file so idk...
    samples: Vec<u8>, // TODO!
}
#[derive(Debug)]
struct SampleInfo {
    sample_name: String,
    sample_length: u16,
    finetune: u8,
    volume: u8,
    repeat_point: u16,
    repeat_length: u16,
}
impl Default for SampleInfo {
    fn default() -> Self {
        Self {
            sample_name: "".to_string(),
            sample_length: 0,
            finetune: 0,
            volume: 0,
            repeat_point: 0,
            repeat_length: 0,
        }
    }
}

impl Default for ModFile {
    fn default() -> Self {
        Self {
            // FIXME!
            song_name: "".to_string(),
            sample_info: Default::default(),
            song_length: 0,
            restart_byte: 0,
            song_position: 0,
            identifier: "".to_string(),
            samples: vec![],
        }
    }
}
impl ModFile {
    pub fn new() -> ModFile {
        let m = Default::default();
        m
    }

    // TODO: make it actually open a file

    // maybe Vec<u8> instead?
    pub fn from_file(&self, f: &[u8]) -> Self {
        // HACK: this might be stupid
        fn z(v: &[u8]) -> String {
            String::from_utf8_lossy(v).to_string()
        };
        // there has to be a better way than this
        Self {
            // TODO: filter the nulls
            song_name: z(&f[SONGNAME_START..SONGNAME_START + SONGNAME_SIZE]),
            // TODO: make this horrible mess less horrible
            sample_info: {
                let mut a = vec![];

                for idx in 0..31 {
                    a.push(SampleInfo {
                        sample_name: z(&f[SAMPLENAME_START + (30 * idx)
                            ..SAMPLENAME_START + (30 * idx) + SAMPLENAME_SIZE]),
                        sample_length: byteorder::BE::read_u16(
                            &f[SAMPLELENGTH_START + (30 * idx)
                                ..SAMPLELENGTH_START + (30 * idx) + SAMPLELENGTH_SIZE],
                        ),
                        finetune: f[REPEATPOINT_START + (30 * idx)],
                        volume: f[VOLUME_START + (30 * idx)],
                        repeat_point: byteorder::BE::read_u16(
                            &f[REPEATPOINT_START + (30 * idx)
                                ..REPEATPOINT_START + (30 * idx) + REPEATPOINT_SIZE],
                        ),
                        repeat_length: byteorder::BE::read_u16(
                            &f[REPEATLENGTH_START + (30 * idx)
                                ..REPEATLENGTH_START + (30 * idx) + REPEATLENGTH_SIZE],
                        ),
                    });
                }

                a
            }, // TODO!
            song_length: f[SONGLENGTH_START],
            restart_byte: f[RESTARTBYTE_START],
            song_position: f[SONGPOSITION_SIZE],
            identifier: z(&f[IDENTIFIER_START..IDENTIFIER_START + IDENTIFIER_SIZE]),

            // TODO: incomplete
            ..Default::default()
        }
    }

    pub fn song_name(&self) -> &String {
        &self.song_name
    }
    pub fn identifier(&self) -> &String {
        &self.identifier
    }
}
