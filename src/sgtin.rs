use crate::{EPCValue, EPC};
use nom::IResult;
use pad::{Alignment, PadStr};

#[derive(PartialEq, Debug)]
pub struct SGTIN96 {
    pub filter: u8,
    pub partition: u8,
    pub company: u64,
    pub item: u64,
    pub serial: u64,
}

impl EPC for SGTIN96 {
    // GS1 EPC section 6.3.1
    fn to_uri(&self) -> String {
        format!(
            "urn:epc:id:sgtin:{}.{}.{}",
            self.company.to_string().pad(
                sgtin_company_digits(self.partition),
                '0',
                Alignment::Right,
                false
            ),
            self.item.to_string(),
            self.serial
        )
    }

    fn to_tag_uri(&self) -> String {
        format!(
            "urn:epc:tag:sgtin-96:{}.{}.{}.{}",
            self.filter,
            self.company.to_string().pad(
                sgtin_company_digits(self.partition),
                '0',
                Alignment::Right,
                false
            ),
            self.item.to_string(),
            self.serial
        )
    }
}

#[derive(PartialEq, Debug)]
pub struct SGTIN198 {
    pub filter: u8,
    pub partition: u8,
    pub company: u64,
    pub item: u64,
    pub serial: str,
}

// Calculate the number of digits in the decimal representation of a SGTIN
// company code from the partition ID.
// EPC GS1 Table 14-2
fn sgtin_company_digits(partition: u8) -> usize {
    12 - partition as usize
}

pub(super) fn decode_sgtin96(data: &[u8]) -> IResult<&[u8], EPCValue> {
    // EPC Table 14-2 and 14-3
    let (data, (filter, (partition, company, item), serial)): (&[u8], (u8, (u8, u64, u64), u64)) = try_parse!(
        data,
        bits!(tuple!(
            take_bits!(3usize),
            switch!(take_bits!(3usize),
                0 => tuple!(
                    value!(0),
                    take_bits!(40usize),
                    take_bits!(4usize)
                )
                | 1 => tuple!(
                    value!(1),
                    take_bits!(37usize),
                    take_bits!(7usize)
                )
                | 2 => tuple!(
                    value!(2),
                    take_bits!(34usize),
                    take_bits!(10usize)
                )
                | 3 => tuple!(
                    value!(3),
                    take_bits!(30usize),
                    take_bits!(14usize)
                )
                | 4 => tuple!(
                    value!(4),
                    take_bits!(27usize),
                    take_bits!(17usize)
                )
                | 5 => tuple!(
                    value!(5),
                    take_bits!(24usize),
                    take_bits!(20usize)
                )
                | 6 => tuple!(
                    value!(6),
                    take_bits!(20usize),
                    take_bits!(24usize)
                )
            ),
            take_bits!(38usize)
        ))
    );

    Ok((
        data,
        EPCValue::SGTIN96(SGTIN96 {
            filter: filter,
            partition: partition,
            company: company,
            item: item,
            serial: serial,
        }),
    ))
}
