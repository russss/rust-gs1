use crate::error::Result;
use bitreader::BitReader;
use pad::{Alignment, PadStr};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use std::cmp;

// General utility functions for working with EPC

// Read an EPC 7-bit ASCII string from the provided BitReader.
// GS1 EPC TDS Section 14.4.2
pub(super) fn read_string(mut reader: BitReader, bits: u64) -> Result<String> {
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

pub(super) fn uri_encode(input: String) -> String {
    utf8_percent_encode(&input, NON_ALPHANUMERIC).to_string()
}

pub(super) fn zero_pad(input: String, digits: usize) -> String {
    input.pad(digits, '0', Alignment::Right, false)
}
