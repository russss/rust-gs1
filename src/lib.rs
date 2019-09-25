//! Library for dealing with GS1 identifiers
//!
//! [GS1](https://www.gs1.org/) is a ubiquitious system for globally unique identifiers in
//! business, and is a superset of better-known standard identifiers such as UPC, EAN, ISBN,
//! and ISSN.
//!
//! GS1 also provides standards for how identifiers should be represented as barcodes and as RFID
//! tags (the Electronic Product Code standard).
//!
//! # About this library
//!
//! This library currently only includes functionality for a limited subset of the GS1 standard,
//! mostly focused around EPCs. Additional functionality is welcome.
//!
//! I'm also not especially familiar with Rust yet, so suggestions on how to structure this code
//! better are greatly appreciated.
//!
//! # Reference
//!
//! The GS1 standards are [freely available](https://www.gs1.org/standards) and code in this
//! library is cross-referenced to these wherever possible.
//!

extern crate bitreader;
extern crate num_enum;
extern crate pad;
extern crate percent_encoding;

use num_enum::IntoPrimitive;
use crate::checksum::gs1_checksum;
use crate::util::zero_pad;

pub mod checksum;
pub mod epc;
pub mod error;

mod util;


// GS1 General Specifications, Figure 3.2-1
#[repr(u16)]
#[derive(Debug, IntoPrimitive)]
pub(crate) enum ApplicationIdentifier {
    SSCC = 0,
    GTIN = 1,
    GTINContent = 2,
    Batch = 10,
    ProductionDate = 11,
    DueDate = 12,
    PackagingDate = 13,
    BestBeforeDate = 15,
    SellByDate = 16,
    ExpirationDate = 17,
    InternalProductVariant = 20,
    SerialNumber = 21
}

/// A GS1 object which is capable of being represented as a GS1 element string.
pub trait GS1 {
    /// Return the GS1 element string for this object.
    ///
    /// Example: `(01) 80614141123458 (21) 6789`
    fn to_gs1(&self) -> String;
}

/// Global Trade Item Number
///
/// This is the most-used GS1 identifier, and is a superset of UPC, EAN, and ISBN codes.
///
/// GS1 General Specifications Section 3.3.2
#[derive(PartialEq, Debug)]
pub struct GTIN {
    /// Company identifier
    pub company: u64,
    /// Number of digits in the decimal representation of the company identifier
    pub company_digits: usize,
    /// Item (product) identifier
    pub item: u64,
    /// Indicator digit in case of GTIN-14, otherwise zero
    pub indicator: u8
}

impl GS1 for GTIN {
    fn to_gs1(&self) -> String {
        let element_string = format!(
            "{}{}{}",
            self.indicator,
            zero_pad(self.company.to_string(), self.company_digits),
            zero_pad(self.item.to_string(), 12 - self.company_digits)
        );
        format!(
            "({:0>2}) {}{}",
            ApplicationIdentifier::GTIN as u16,
            element_string,
            gs1_checksum(&element_string),
        )
    }
}
