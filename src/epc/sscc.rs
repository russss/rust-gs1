use crate::checksum::gs1_checksum;
use crate::epc::util::{extract_indicator, zero_pad};
use crate::epc::{EPCValue, EPC, GS1};
use crate::error::Result;
use crate::general::ApplicationIdentifier;
use bitreader::BitReader;

#[derive(PartialEq, Debug)]
pub struct SSCC96 {
    pub filter: u8,
    pub partition: u8,
    pub indicator: u8,
    pub company: u64,
    pub serial: u64,
}

impl EPC for SSCC96 {
    // GS1 EPC TDS section 6.3.1
    fn to_uri(&self) -> String {
        format!(
            "urn:epc:id:sscc:{}.{}{}",
            zero_pad(self.company.to_string(), company_digits(self.partition)),
            self.indicator,
            zero_pad(self.serial.to_string(), item_digits(self.partition) - 1)
        )
    }

    fn to_tag_uri(&self) -> String {
        format!(
            "urn:epc:tag:sscc-96:{}.{}.{}{}",
            self.filter,
            zero_pad(self.company.to_string(), company_digits(self.partition)),
            self.indicator,
            zero_pad(self.serial.to_string(), item_digits(self.partition) - 1)
        )
    }

    fn get_value(&self) -> EPCValue {
        EPCValue::SSCC96(self)
    }
}

impl GS1 for SSCC96 {
    fn to_gs1(&self) -> String {
        let element_string = format!(
            "{}{}{}",
            self.indicator,
            zero_pad(self.company.to_string(), company_digits(self.partition)),
            zero_pad(self.serial.to_string(), item_digits(self.partition) - 1)
        );
        format!(
            "({:0>2}) {}{}",
            ApplicationIdentifier::SSCC as u16,
            element_string,
            gs1_checksum(&element_string)
        )
    }
}

// Calculate the number of digits in the decimal representation of a SGTIN
// company code from the partition ID.
// GS1 EPC TDS Table 14-5
fn company_digits(partition: u8) -> usize {
    12 - partition as usize
}

fn item_digits(partition: u8) -> usize {
    17 - company_digits(partition)
}

// GS1 EPC TDS Table 14-5
fn partition_bits(partition: u8) -> Result<(u8, u8)> {
    Ok(match partition {
        0 => (40, 18),
        1 => (37, 21),
        2 => (34, 24),
        3 => (30, 28),
        4 => (27, 31),
        5 => (24, 34),
        6 => (20, 48),
        _ => {
            panic!(format!("Invalid partition value: {}", partition));
            // TODO: error
            //return Err("Invalid partition value");
        }
    })
}

// GS1 EPC TDC Section 14.5.2
pub(super) fn decode_sscc96(data: &[u8]) -> Result<Box<dyn EPC>> {
    let mut reader = BitReader::new(data);

    let filter = reader.read_u8(3)?;
    let partition = reader.read_u8(3)?;
    let (company_bits, serial_bits) = partition_bits(partition)?;
    let company = reader.read_u64(company_bits)?;
    let serial = reader.read_u64(serial_bits)?;
    let (serial, indicator) = extract_indicator(serial, item_digits(partition))?;

    Ok(Box::new(SSCC96 {
        filter: filter,
        partition: partition,
        indicator: indicator,
        company: company,
        serial: serial,
    }))
}
