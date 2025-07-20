// TODO: better naming

// Trailing null bytes at the end
pub const SONGNAME_START: usize = 0;
pub const SONGNAME_SIZE: usize = 20;

// SAMPLE

// Padded with null bytes
pub const SAMPLENAME_START: usize = 20;
pub const SAMPLENAME_SIZE: usize = 22;

// Byte order: Big Endian
// Multiply by two to get real sample length in bytes
pub const SAMPLELENGTH_START: usize = 42;
pub const SAMPLELENGTH_SIZE: usize = 2;

// Lower four bits are the finetune value, stored as a signed
// four bit number. The upper four bits are not used, and
// should be set to zero.
pub const FINETUNE_START: usize = 44;
pub const FINETUNE_SIZE: usize = 1;

// Volme range is 0..64
pub const VOLUME_START: usize = 45;
pub const VOLUME_SIZE: usize = 1;

// Endianess: BE; multiply by 2
pub const REPEATPOINT_START: usize = 46;
pub const REPEATPOINT_SIZE: usize = 2;

// Endianess: BE; multiply by 2
pub const REPEATLENGTH_START: usize = 48;
pub const REPEATLENGTH_SIZE: usize = 2;

// TODO: patterns

pub const SONGLENGTH_START: usize = 950;
// For stuff like 1 byte sized stuff maybe we should just not
// write the _SIZE entries anymore?
pub const SONGLENGTH_SIZE: usize = 1;

// TODO!
pub const RESTARTBYTE_START: usize = 951;

// either 0..63 or 0..127
pub const SONGPOSITION_START: usize = 952;
pub const SONGPOSITION_SIZE: usize = 128;

// TODO: anything else?
// 'M.K.' => 31 samples
// 'M!K!' => there are more than 64 patterns
// _ => 15 samples
pub const IDENTIFIER_START: usize = 1080;
pub const IDENTIFIER_SIZE: usize = 4;

// Anything past byte 1084 will contain samples
pub const SAMPLES_START: usize = 1084;
