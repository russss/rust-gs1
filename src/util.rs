use crate::error::Result;
use bitreader::BitReader;
use pad::{Alignment, PadStr};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::cmp;

// General utility functions for working with EPC

// Read an EPC 7-bit ASCII string from the provided BitReader.
// GS1 EPC TDS Section 14.4.2
pub(crate) fn read_string(mut reader: BitReader, bits: u64) -> Result<String> {
    let num_chars = cmp::min(reader.remaining(), bits) / 7;
    let mut chars: Vec<char> = Vec::new();

    for _i in 0..num_chars {
        let value = reader.read_u8(7)?;
        if value != 0 {
            chars.push(value as char);
        }
    }

    Ok(chars.iter().collect())
}

pub(crate) fn uri_encode(input: String) -> String {
    utf8_percent_encode(&input, NON_ALPHANUMERIC).to_string()
}

pub(crate) fn zero_pad(input: String, digits: usize) -> String {
    input.pad(digits, '0', Alignment::Right, false)
}

pub(crate) fn extract_indicator(item: u64, item_digits: usize) -> Result<(u64, u8)> {
    // The first character of the correctly-padded item string is the indicator digit or must be
    // zero. I think.
    // This is not terribly well spelled out in the GS1 EPC spec.
    //
    // TODO: error handling could be improved, but in practice most of these errors are probably
    // unreachable.
    let item_str = zero_pad(item.to_string(), item_digits);
    let mut item_str_iterator = item_str.chars();
    let indicator = item_str_iterator.next().unwrap().to_digit(10).unwrap() as u8;
    let item = item_str_iterator.collect::<String>().parse::<u64>()?;
    Ok((item, indicator))
}
