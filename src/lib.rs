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

extern crate num_enum;
extern crate bitreader;
extern crate pad;
extern crate percent_encoding;

pub mod epc;
pub mod checksum;
pub mod error;
mod general;

/// A GS1 object which is capable of being represented as a GS1 code (i.e. a barcode).
pub trait GS1 {
    /// Return the GS1 code string for this object.
    ///
    /// Example: `(01) 80614141123458 (21) 6789`
    fn to_gs1(&self) -> String;
}

