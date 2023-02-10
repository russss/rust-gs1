//! General Identifier
//!
//! This is a combination of manager number assigned by GS1, an object class
//! assigned by that mananger, and a serial number which allows an item to
//! be uniquely identfied.
use crate::epc::{EPCValue, EPC};
use crate::error::Result;
use bitreader::BitReader;

/// 96-bit General Identifier
///
/// This comprises a manager number, an object class, and a numeric serial
/// number.
#[derive(PartialEq, Debug)]
pub struct GID96 {
    /// General Manager Number
    pub manager: u32,
    /// Object Class
    pub class: u32,
    /// Item serial number
    pub serial: u64,
}

impl EPC for GID96 {
    // GS1 EPC TDS section 6.3.16
    fn to_uri(&self) -> String {
        format!(
            "urn:epc:id:gid:{}.{}.{}",
            self.manager, self.class, self.serial
        )
    }

    fn to_tag_uri(&self) -> String {
        format!(
            "urn:epc:tag:gid-96:{}.{}.{}",
            self.manager, self.class, self.serial
        )
    }

    fn get_value(&self) -> EPCValue {
        EPCValue::GID96(self)
    }
}

// GS1 EPC TDS Section 14.6.12
pub(super) fn decode_gid96(data: &[u8]) -> Result<Box<dyn EPC>> {
    let mut reader = BitReader::new(data);

    let manager = reader.read_u32(28)?;
    let class = reader.read_u32(24)?;
    let serial = reader.read_u64(36)?;

    Ok(Box::new(GID96 {
        manager,
        class,
        serial,
    }))
}
