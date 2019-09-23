use crate::{EPC, EPCValue, GS1, gs1::ApplicationIdentifier};
use nom::IResult;
use pad::{Alignment, PadStr};

fn zero_pad(input: String, digits: usize) -> String {
    input.pad(digits, '0', Alignment::Right, false)
}

fn int_digits(input: &String) -> Vec<u16> {
    input.chars().map(|d| d.to_digit(10).unwrap() as u16).collect()
}

fn gs1_checksum(input: &String) -> u8 {
    let digits = int_digits(input);
    let mut even: u16 = 0;
    let mut odd: u16 = 0;

    for i in 1..digits.len() + 1 {
        let curr = digits[digits.len() - i];
        if i % 2 == 0 {
            even += curr;
        } else {
            odd += curr;
        }
    }

    let mut check = (3 * odd + even) % 10;
    if check > 0 {
        check = 10 - check;
    }

    check as u8
}

#[derive(PartialEq, Debug)]
pub struct SGTIN96 {
    pub filter: u8,
    pub partition: u8,
    pub company: u64,
    pub indicator: u8,
    pub item: u64,
    pub serial: u64,
}

impl EPC for SGTIN96 {
    // GS1 EPC TDS section 6.3.1
    fn to_uri(&self) -> String {
        format!(
            "urn:epc:id:sgtin:{}.{}{}.{}",
            zero_pad(self.company.to_string(), sgtin_company_digits(self.partition)),
            self.indicator.to_string(),
            zero_pad(self.item.to_string(), sgtin_item_digits(self.partition) - 1),
            self.serial
        )
    }

    fn to_tag_uri(&self) -> String {
        format!(
            "urn:epc:tag:sgtin-96:{}.{}.{}{}.{}",
            self.filter,
            zero_pad(self.company.to_string(), sgtin_company_digits(self.partition)),
            self.indicator.to_string(),
            zero_pad(self.item.to_string(), sgtin_item_digits(self.partition) - 1),
            self.serial
        )
    }

    fn get_value(&self) -> EPCValue {
        EPCValue::SGTIN96(self)
    }
}

impl GS1 for SGTIN96 {
    fn to_gs1(&self) -> String {
        let element_string = format!("{}{}{}",
            self.indicator,
            zero_pad(self.company.to_string(), sgtin_company_digits(self.partition)),
            zero_pad(self.item.to_string(), sgtin_item_digits(self.partition) - 1)
        );
        format!(
            "({:0>2}) {}{} ({:0>2}) {}",
            ApplicationIdentifier::GTIN as u16,
            element_string,
            gs1_checksum(&element_string),
            ApplicationIdentifier::SerialNumber as u16,
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
// GS1 EPC TDS Table 14-2
fn sgtin_company_digits(partition: u8) -> usize {
    12 - partition as usize
}

fn sgtin_item_digits(partition: u8) -> usize {
    13 - sgtin_company_digits(partition)
}

pub(super) fn decode_sgtin96(data: &[u8]) -> IResult<&[u8], Box<dyn EPC>> {
    // GS1 EPC TDS Table 14-2 and 14-3
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

    // The first character of the correctly-padded item string is the indicator digit or must be
    // zero. I think.
    // This is not terribly well spelled out in the GS1 EPC spec.
    let item_str = zero_pad(item.to_string(), sgtin_item_digits(partition));
    let mut item_str_iterator = item_str.chars();
    let indicator = item_str_iterator.next().unwrap().to_digit(10).unwrap() as u8;
    let item = item_str_iterator.collect::<String>().parse::<u64>().unwrap();
    
    Ok((
        data,
        Box::new(SGTIN96 {
            filter: filter,
            partition: partition,
            company: company,
            item: item,
            indicator: indicator,
            serial: serial,
        }),
    ))
}


#[test]
fn test_gs1_checksum() {
    assert_eq!(0, gs1_checksum(&"0360843951968".to_string()));
    assert_eq!(8, gs1_checksum(&"8061414112345".to_string()));
}

