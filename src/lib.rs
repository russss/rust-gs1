//! Library for dealing with GS1 identifiers
//!
//! 

extern crate num_enum;
extern crate bitreader;
extern crate pad;
extern crate percent_encoding;

pub mod epc;
pub mod checksum;
pub mod error;
mod general;
