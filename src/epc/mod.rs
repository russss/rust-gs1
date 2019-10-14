//! GS1 Electronic Product Codes
//!
//! EPCs are used to represent GS1 IDs on Gen2 RFID tags.
//! This is documented in the [GS1 EPC Tag Data Standard](https://www.gs1.org/standards/epc-rfid/tds).
//!
use crate::error::{Result, UnimplementedError};
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

pub mod sgtin;
pub mod sscc;
pub mod tid;


// EPC Table 14-1
#[derive(Debug, Eq, PartialEq, TryFromPrimitive, Copy, Clone)]
#[repr(u8)]
enum EPCBinaryHeader {
    Unprogrammed = 0x00,
    GTDI96 = 0x2C,
    GSRN96 = 0x2D,
    GSRNP = 0x2E,
    USDoD96 = 0x2F,
    SGITN96 = 0x30,
    SSCC96 = 0x31,
    SGLN96 = 0x32,
    GRAI96 = 0x33,
    GIAI96 = 0x34,
    GID96 = 0x35,
    SGITN198 = 0x36,
    GRAI170 = 0x37,
    GIAI202 = 0x38,
    SGLN195 = 0x39,
    GTDI113 = 0x3A,
    ADIVAR = 0x3B,
    CPI96 = 0x3C,
    CPIVAR = 0x3D,
    GDTI174 = 0x3E,
    SGCN96 = 0x3F,
    ITIP110 = 0x40,
    ITIP212 = 0x41,
}

/// A GS1 object which is capable of being represented as an EPC.
pub trait EPC {
    /// Return the EPC pure identity URI for this object.
    ///
    /// Example: `urn:epc:id:sgtin:0614141.812345.6789`
    fn to_uri(&self) -> String;
    /// Return the EPC tag URI for this object.
    ///
    /// This URI includes all data from the pure URI, plus tag-specific data which does not form
    /// part of the identifier.
    ///
    /// Example: `urn:epc:tag:sgtin-96:3.0614141.812345.6789`
    fn to_tag_uri(&self) -> String;
    /// Return the underlying EPC structure in an `EPCValue` tagged enum.
    fn get_value(&self) -> EPCValue;
}

/// Represents an unprogrammed tag (with the header byte 0x00)
#[derive(PartialEq, Debug)]
pub struct Unprogrammed {
    pub data: Vec<u8>,
}

impl EPC for Unprogrammed {
    fn to_uri(&self) -> String {
        "urn:epc:id:unprogrammed".to_string()
    }

    fn to_tag_uri(&self) -> String {
        "urn:epc:tag:unprogrammed".to_string()
    }

    fn get_value(&self) -> EPCValue {
        EPCValue::Unprogrammed(self)
    }
}

/// A tagged union to allow data structures to be returned from the EPC trait
#[derive(PartialEq, Debug)]
pub enum EPCValue<'a> {
    Unprogrammed(&'a Unprogrammed),
    SGTIN96(&'a sgtin::SGTIN96),
    SGTIN198(&'a sgtin::SGTIN198),
    SSCC96(&'a sscc::SSCC96),
}

fn take_header(data: &[u8]) -> Result<(&[u8], EPCBinaryHeader)> {
    let header = EPCBinaryHeader::try_from(data[0])?;
    Ok((&data[1..], header))
}

/// Decode a binary EPC code, as received from an RFID tag.
pub fn decode_binary(data: &[u8]) -> Result<Box<dyn EPC>> {
    let (data, header) = take_header(data)?;

    Ok(match header {
        EPCBinaryHeader::SGITN96 => sgtin::decode_sgtin96(data)?,
        EPCBinaryHeader::SGITN198 => sgtin::decode_sgtin198(data)?,
        EPCBinaryHeader::SSCC96 => sscc::decode_sscc96(data)?,
        EPCBinaryHeader::Unprogrammed => 
            Box::new(Unprogrammed {
                data: data.to_vec(),
            }) as Box<dyn EPC>,
        _unimplemented => {
            return Err(Box::new(UnimplementedError()));
        }
    })
}
