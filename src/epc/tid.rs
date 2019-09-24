//! Decoder for EPC Tag Identification
//!
//! # Reference
//! GS1 EPC TDS Section 16
use bitreader::{BitReader, BitReaderError};

#[derive(PartialEq, Debug)]
pub struct TID {
    pub xtid: bool,
    pub security: bool,
    pub file: bool,
    pub mdid: u16,
    pub tmid: u16
}

// GS1 EPC TDS Section 16
pub fn decode_tid(data: &[u8]) -> Result<TID, BitReaderError> {
    let mut reader = BitReader::new(data);
    if reader.read_u8(8)? != 0xE2 {
        panic!("Fix this");
    }

    Ok(TID {
        xtid: reader.read_bool()?,
        security: reader.read_bool()?,
        file: reader.read_bool()?,
        mdid: reader.read_u16(9)?,
        tmid: reader.read_u16(12)?
    })
}
