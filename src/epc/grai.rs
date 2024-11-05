//! Global Returnable Asset Identifier
//!
//! This is a combination of a company prefix assigned by GS1, an asset type
//! assigned by that company, and a serial number which allows an item to
//! be uniquely identified.
use crate::epc::{EPCValue, EPC};
use crate::error::Result;
use bitreader::BitReader;

/// Metadata for a partition
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
struct Partition {
    bits: u8,
    digits: u8,
}

/// Decoded partition metadata
#[derive(Debug, PartialEq)]
struct GraiPartition {
    company_prefix: Partition,
    asset_type: Partition,
}

/// Get the number of bits and digits for the company prefix and the asset type based on the partition value.
///
/// Decoded according to Table 14-14 "GRAI Partition Table" in the GS1 EPC Tag Data Standard v2.1
fn decode_partition_value(partition_value: u8) -> Result<GraiPartition> {
    match partition_value {
        0 => Ok(GraiPartition {
            company_prefix: Partition {
                bits: 40,
                digits: 12,
            },
            asset_type: Partition { bits: 4, digits: 0 },
        }),
        1 => Ok(GraiPartition {
            company_prefix: Partition {
                bits: 37,
                digits: 11,
            },
            asset_type: Partition { bits: 7, digits: 1 },
        }),
        2 => Ok(GraiPartition {
            company_prefix: Partition {
                bits: 34,
                digits: 10,
            },
            asset_type: Partition {
                bits: 10,
                digits: 2,
            },
        }),
        3 => Ok(GraiPartition {
            company_prefix: Partition {
                bits: 30,
                digits: 9,
            },
            asset_type: Partition {
                bits: 14,
                digits: 3,
            },
        }),
        4 => Ok(GraiPartition {
            company_prefix: Partition {
                bits: 27,
                digits: 8,
            },
            asset_type: Partition {
                bits: 17,
                digits: 4,
            },
        }),
        5 => Ok(GraiPartition {
            company_prefix: Partition {
                bits: 24,
                digits: 7,
            },
            asset_type: Partition {
                bits: 20,
                digits: 5,
            },
        }),
        6 => Ok(GraiPartition {
            company_prefix: Partition {
                bits: 20,
                digits: 6,
            },
            asset_type: Partition {
                bits: 24,
                digits: 6,
            },
        }),
        _ => Err("Invalid partition value".into()),
    }
}

// EPC Header Filter Partition GS1
// Company
// Prefix
// Asset Type Serial

/// 96-bit Global Returnable Asset Identifier
///
/// This comprises a manager number, an object class, and a numeric serial
/// number.
#[derive(PartialEq, Debug)]
pub struct GRAI96 {
    /// Filter
    pub filter: u8,
    /// Partition
    pub partition: u8,
    /// GS1 Company Prefix
    pub company_prefix: u64,
    /// Asset type
    pub asset_type: u32,
    /// Serial number
    pub serial: u64,
}

impl EPC for GRAI96 {
    // GS1 EPC TDS section 14.6.4
    fn to_uri(&self) -> String {
        format!(
            "urn:epc:id:grai:{}.{}.{}",
            self.company_prefix, self.asset_type, self.serial
        )
    }

    fn to_tag_uri(&self) -> String {
        format!(
            "urn:epc:tag:grai-96:{}.{}.{}.{}",
            self.filter, self.company_prefix, self.asset_type, self.serial
        )
    }

    fn get_value(&self) -> EPCValue {
        EPCValue::GRAI96(self)
    }
}

// GS1 EPC TDS Section 14.6.4
pub fn decode_grai96(data: &[u8]) -> Result<Box<dyn EPC>> {
    let mut reader = BitReader::new(data);

    let filter = reader.read_u8(3)?;
    let partition = reader.read_u8(3)?;

    let grai_partition = decode_partition_value(partition)?;

    let company_prefix = reader.read_u64(grai_partition.company_prefix.bits)?;
    let asset_type = reader.read_u32(grai_partition.asset_type.bits)?;
    let serial = reader.read_u64(38)?;

    Ok(Box::new(GRAI96 {
        filter,
        partition,
        company_prefix,
        asset_type,
        serial,
    }))
}
