//! Decoder for EPC Tag Identification data
//!
//! The TID is a memory field on a Gen2 RFID tag which represents the capabilities of the tag.
//!
//! # Reference
//! GS1 EPC TDS Section 16
use crate::error::{Result, ParseError};
use bitreader::BitReader;

/// EPC Tag Identification
#[derive(PartialEq, Debug)]
pub struct TID {
    /// Whether the Tag implements Extended Tag Identification
    pub xtid: bool,
    /// Whether the Tag supports the Authenticate and/or Challenge commands
    pub security: bool,
    /// Whether the Tag supports the FileOpen command
    pub file: bool,
    /// Mask-designer identifier
    pub mdid: u16,
    /// Tag-manufacturer-defined Tag Model Number
    pub tmid: u16,
}

/// Decode the TID structure from bytes. 
///
/// Reference: GS1 EPC TDS Section 16
pub fn decode_tid(data: &[u8]) -> Result<TID> {
    let mut reader = BitReader::new(data);
    if reader.read_u8(8)? != 0xE2 {
        return Err(Box::new(ParseError()));
    }

    Ok(TID {
        xtid: reader.read_bool()?,
        security: reader.read_bool()?,
        file: reader.read_bool()?,
        mdid: reader.read_u16(9)?,
        tmid: reader.read_u16(12)?,
    })
}
