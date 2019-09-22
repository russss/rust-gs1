extern crate num_enum;

#[macro_use]
extern crate nom;

use nom::combinator::map_res;
use nom::number::complete::le_u8;
use nom::IResult;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;

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

#[derive(PartialEq, Debug)]
pub struct SGTIN96 {
    pub filter: u8,
    pub company: u64,
    pub item: u64,
    pub serial: u64,
}

#[derive(PartialEq, Debug)]
pub struct Unprogrammed {
    pub data: Vec<u8>,
}

#[derive(PartialEq, Debug)]
pub enum EPC {
    Unprogrammed(Unprogrammed),
    SGTIN96(SGTIN96),
}

fn take_header(data: &[u8]) -> IResult<&[u8], EPCBinaryHeader> {
    map_res(le_u8, EPCBinaryHeader::try_from)(data)
}

fn decode_sgtin96(data: &[u8]) -> IResult<&[u8], EPC> {
    // EPC Table 14-2 and 14-3
    let (data, (filter, (company, item), serial)): (&[u8], (u8, (u64, u64), u64)) = try_parse!(
        data,
        bits!(tuple!(
            take_bits!(3usize),
            switch!(take_bits!(3usize),
                0 => tuple!(
                    take_bits!(40usize),
                    take_bits!(4usize)
                )
                | 1 => tuple!(
                    take_bits!(37usize),
                    take_bits!(7usize)
                )
                | 2 => tuple!(
                    take_bits!(34usize),
                    take_bits!(10usize)
                )
                | 3 => tuple!(
                    take_bits!(30usize),
                    take_bits!(14usize)
                )
                | 4 => tuple!(
                    take_bits!(27usize),
                    take_bits!(17usize)
                )
                | 5 => tuple!(
                    take_bits!(24usize),
                    take_bits!(20usize)
                )
                | 6 => tuple!(
                    take_bits!(20usize),
                    take_bits!(24usize)
                )
            ),
            take_bits!(38usize)
        ))
    );

    Ok((
        data,
        EPC::SGTIN96(SGTIN96 {
            filter: filter,
            company: company,
            item: item,
            serial: serial,
        }),
    ))
}

pub fn decode(data: &[u8]) -> IResult<&[u8], EPC> {
    let (data, header) = take_header(data)?;

    let (data, epc) = match header {
        EPCBinaryHeader::SGITN96 => decode_sgtin96(data)?,
        EPCBinaryHeader::Unprogrammed => (
            &[] as &[u8],
            EPC::Unprogrammed(Unprogrammed {
                data: data.to_vec(),
            }),
        ),
        unimplemented => {
            panic!("Unimplemented EPC type {:?}", unimplemented);
        }
    };

    Ok((data, epc))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        let data = [48, 57, 96, 98, 195, 161, 168, 0, 0, 107, 51, 244];
        println!("{:?}", decode(&data));

        let data = [0, 176, 122, 20, 12, 95, 156, 81, 64, 0, 3, 238];
        println!("{:?}", decode(&data));

        let data = [226, 0, 0, 25, 6, 12, 2, 9, 6, 144, 211, 194];
        println!("{:?}", decode(&data));
    }
}
