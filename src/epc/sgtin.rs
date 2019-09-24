use crate::checksum::gs1_checksum;
use crate::epc::util::{extract_indicator, read_string, uri_encode, zero_pad};
use crate::epc::{EPCValue, EPC, GS1};
use crate::error::Result;
use crate::general::ApplicationIdentifier;
use bitreader::BitReader;

#[derive(PartialEq, Debug)]
pub struct SGTIN96 {
    pub filter: u8,
    pub partition: u8,
    pub indicator: u8,
    pub company: u64,
    pub item: u64,
    pub serial: u64,
}

impl EPC for SGTIN96 {
    // GS1 EPC TDS section 6.3.1
    fn to_uri(&self) -> String {
        format!(
            "urn:epc:id:sgtin:{}.{}{}.{}",
            zero_pad(self.company.to_string(), company_digits(self.partition)),
            self.indicator.to_string(),
            zero_pad(self.item.to_string(), item_digits(self.partition) - 1),
            self.serial
        )
    }

    fn to_tag_uri(&self) -> String {
        format!(
            "urn:epc:tag:sgtin-96:{}.{}.{}{}.{}",
            self.filter,
            zero_pad(self.company.to_string(), company_digits(self.partition)),
            self.indicator.to_string(),
            zero_pad(self.item.to_string(), item_digits(self.partition) - 1),
            self.serial
        )
    }

    fn get_value(&self) -> EPCValue {
        EPCValue::SGTIN96(self)
    }
}

impl GS1 for SGTIN96 {
    fn to_gs1(&self) -> String {
        let element_string = format!(
            "{}{}{}",
            self.indicator,
            zero_pad(self.company.to_string(), company_digits(self.partition)),
            zero_pad(self.item.to_string(), item_digits(self.partition) - 1)
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
    pub indicator: u8,
    pub company: u64,
    pub item: u64,
    pub serial: String,
}

impl EPC for SGTIN198 {
    // GS1 EPC TDS section 6.3.1
    fn to_uri(&self) -> String {
        format!(
            "urn:epc:id:sgtin:{}.{}{}.{}",
            zero_pad(self.company.to_string(), company_digits(self.partition)),
            self.indicator,
            zero_pad(self.item.to_string(), item_digits(self.partition) - 1),
            uri_encode(self.serial.to_string())
        )
    }

    fn to_tag_uri(&self) -> String {
        format!(
            "urn:epc:tag:sgtin-198:{}.{}.{}{}.{}",
            self.filter,
            zero_pad(self.company.to_string(), company_digits(self.partition)),
            self.indicator,
            zero_pad(self.item.to_string(), item_digits(self.partition) - 1),
            uri_encode(self.serial.to_string())
        )
    }

    fn get_value(&self) -> EPCValue {
        EPCValue::SGTIN198(self)
    }
}

impl GS1 for SGTIN198 {
    fn to_gs1(&self) -> String {
        let element_string = format!(
            "{}{}{}",
            self.indicator,
            zero_pad(self.company.to_string(), company_digits(self.partition)),
            zero_pad(self.item.to_string(), item_digits(self.partition) - 1)
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

// Calculate the number of digits in the decimal representation of a SGTIN
// company code from the partition ID.
// GS1 EPC TDS Table 14-2
fn company_digits(partition: u8) -> usize {
    12 - partition as usize
}

fn item_digits(partition: u8) -> usize {
    13 - company_digits(partition)
}

// GS1 EPC TDS Table 14-2
fn partition_bits(partition: u8) -> Result<(u8, u8)> {
    Ok(match partition {
        0 => (40, 4),
        1 => (37, 7),
        2 => (34, 10),
        3 => (30, 14),
        4 => (27, 17),
        5 => (24, 20),
        6 => (20, 24),
        _ => {
            panic!(format!("Invalid partition value: {}", partition));
            // TODO: error
            //return Err("Invalid partition value");
        }
    })
}

// GS1 EPC TDC Section 14.5.1
pub(super) fn decode_sgtin96(data: &[u8]) -> Result<Box<dyn EPC>> {
    let mut reader = BitReader::new(data);

    let filter = reader.read_u8(3)?;
    let partition = reader.read_u8(3)?;
    let (company_bits, item_bits) = partition_bits(partition)?;
    let company = reader.read_u64(company_bits)?;
    let item = reader.read_u64(item_bits)?;
    let (item, indicator) = extract_indicator(item, item_digits(partition))?;
    let serial = reader.read_u64(38)?;

    Ok(Box::new(SGTIN96 {
        filter: filter,
        partition: partition,
        company: company,
        item: item,
        indicator: indicator,
        serial: serial,
    }))
}

// GS1 EPC TDC Section 14.5.1.2
pub(super) fn decode_sgtin198(data: &[u8]) -> Result<Box<dyn EPC>> {
    let mut reader = BitReader::new(data);

    let filter = reader.read_u8(3)?;
    let partition = reader.read_u8(3)?;
    let (company_bits, item_bits) = partition_bits(partition)?;
    let company = reader.read_u64(company_bits)?;
    let item = reader.read_u64(item_bits)?;
    let (item, indicator) = extract_indicator(item, item_digits(partition))?;
    let serial = read_string(reader, 140)?;

    Ok(Box::new(SGTIN198 {
        filter: filter,
        partition: partition,
        company: company,
        item: item,
        indicator: indicator,
        serial: serial,
    }))
}
