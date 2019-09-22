extern crate num_enum;
extern crate pad;

#[macro_use]
extern crate nom;

use nom::combinator::map_res;
use nom::number::complete::le_u8;
use nom::IResult;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

pub mod sgtin;

// EPC Table 14-1
#[derive(Debug, Eq, PartialEq, TryFromPrimitive)]
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
}

#[derive(PartialEq, Debug)]
pub enum EPCValue {
    Unprogrammed(Unprogrammed),
    SGTIN96(sgtin::SGTIN96),
}

fn take_header(data: &[u8]) -> IResult<&[u8], EPCBinaryHeader> {
    map_res(le_u8, EPCBinaryHeader::try_from)(data)
}


fn decode(data: &[u8]) -> IResult<&[u8], EPCValue> {
    let (data, header) = take_header(data)?;

    let (data, epc) = match header {
        EPCBinaryHeader::SGITN96 => sgtin::decode_sgtin96(data)?,
        EPCBinaryHeader::Unprogrammed => (
            &[] as &[u8],
            EPCValue::Unprogrammed(Unprogrammed {
                data: data.to_vec(),
            }),
        ),
        unimplemented => {
            panic!("Unimplemented EPC type {:?}", unimplemented);
        }
    };

    Ok((data, epc))
}

pub fn decode_binary(data: &[u8]) -> Result<EPCValue, String> {
    match decode(data) {
        Ok((_data, epc)) => Ok(epc),
        Err(error) => Err(format!("Unable to parse binary EPC: {:?}", error))
    }
}

pub fn decode_binary_box(data: &[u8]) -> Result<Box<dyn EPC>, String> {
    let val: Box<dyn EPC> = match decode_binary(data)? {
        EPCValue::SGTIN96(val) => Box::new(val),
        EPCValue::Unprogrammed(val) => Box::new(val)
    };
    Ok(val)
}
