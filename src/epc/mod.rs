use crate::error::Result;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

pub mod sgtin;
pub mod tid;
mod util;

// EPC Table 14-1
#[derive(Debug, Eq, PartialEq, TryFromPrimitive, Copy, Clone)]
#[repr(u8)]
pub enum EPCBinaryHeader {
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

pub trait EPC {
    fn to_uri(&self) -> String;
    fn to_tag_uri(&self) -> String;
    fn get_value(&self) -> EPCValue;
}

pub trait GS1 {
    fn to_gs1(&self) -> String;
}

#[derive(PartialEq, Debug)]
pub struct Unprogrammed {
    pub data: Vec<u8>,
}

impl EPC for Unprogrammed {
    fn to_uri(&self) -> String {
        format!("urn:epc:id:unprogrammed")
    }

    fn to_tag_uri(&self) -> String {
        format!("urn:epc:tag:unprogrammed")
    }

    fn get_value(&self) -> EPCValue {
        EPCValue::Unprogrammed(self)
    }
}

#[derive(PartialEq, Debug)]
pub enum EPCValue<'a> {
    Unprogrammed(&'a Unprogrammed),
    SGTIN96(&'a sgtin::SGTIN96),
    SGTIN198(&'a sgtin::SGTIN198),
}

fn take_header(data: &[u8]) -> Result<(&[u8], EPCBinaryHeader)> {
    let header = EPCBinaryHeader::try_from(data[0])?;
    Ok((&data[1..], header))
}

pub fn decode_binary(data: &[u8]) -> Result<Box<dyn EPC>> {
    let (data, header) = take_header(data)?;

    let epc = match header {
        EPCBinaryHeader::SGITN96 => sgtin::decode_sgtin96(data)?,
        EPCBinaryHeader::SGITN198 => sgtin::decode_sgtin198(data)?,
        EPCBinaryHeader::Unprogrammed => 
            Box::new(Unprogrammed {
                data: data.to_vec(),
            }) as Box<dyn EPC>,
        unimplemented => {
            panic!("Unimplemented EPC type {:?}", unimplemented);
        }
    };

    Ok(epc)
}
