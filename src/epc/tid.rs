//! Decoder for EPC Tag Identification data
//!
//! The TID is a memory area on a Gen2 RFID tag which contains the manufacturer identification
//! and capabilities of the tag.
//!
//! As Gen2 tags will refuse an out-of-bounds read, the TID memory must be read from the tag and
//! decoded progressively. Decoding the TID structure (4 bytes / 2 words) will indicate whether
//! the XTID header (2 bytes / 1 word) is present and can be read. The XTID header then determines
//! which of the subsequent data structures are present.
//!
//! # Reference
//! GS1 EPC TDS Section 16
use crate::error::{ParseError, Result};
use bitreader::BitReader;

/// Tag Identification
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct TID {
    /// Whether the Tag implements Extended Tag Identification
    pub xtid: bool,
    /// Whether the Tag supports the Authenticate and/or Challenge commands
    pub security: bool,
    /// Whether the Tag supports the FileOpen command
    pub file: bool,
    /// Mask-designer (manufacturer) identifier
    pub mdid: u16,
    /// Tag-manufacturer-defined Tag Model Number
    pub tmid: u16,
}

/// Decode the TID structure from bytes
///
/// The TID structure is held in the first 4 bytes (2 words) of the TID memory.
///
/// Reference: GS1 EPC TDS Section 16.2
pub fn decode_tid(data: &[u8]) -> Result<TID> {
    let mut reader = BitReader::new(data);
    if reader.read_u8(8)? != 0xE2 {
        return Err(Box::new(ParseError()));
    }

    Ok(TID {
        xtid: reader.read_bool()?,
        security: reader.read_bool()?,
        file: reader.read_bool()?,
        mdid: reader.read_u16(9)?,
        tmid: reader.read_u16(12)?,
    })
}

/// Extended Tag ID header
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct XTIDHeader {
    /// Whether a further XTID header is present - always false
    pub extended_header: bool,
    /// Whether the XTID includes the User Memory and Block PermaLock segment
    pub user_memory_permalock: bool,
    /// Whether the XTID includes the BlockWrite and BlockErase segment
    pub blockwrite_blockerase: bool,
    /// Whether the XTID includes the Optional Command Support segment
    pub optional_command_support: bool,
    /// The serial number size, in bits
    pub serial_size: u16,
}

/// Decode the XTID header from bytes
///
/// The XTID structure is 2 bytes (1 word) long and starts from the 5th byte (3rd word) of the TID
/// memory, if the XTID bit is set in the TID structure.
///
/// Reference: GS1 EPC TDS Section 16.2.1
pub fn decode_xtid_header(data: &[u8]) -> Result<XTIDHeader> {
    let mut reader = BitReader::new(data);

    let extended_header = reader.read_bool()?;
    // Reserved for future use bits - should be zero but it seems like they frequently aren't.
    let _rfu = reader.read_u16(9)?;
    /*
    if rfu != 0 {
        println!("RFU: {:?}", rfu);
        return Err(Box::new(ParseError()));
    }*/
    let user_memory_permalock = reader.read_bool()?;
    let blockwrite_blockerase = reader.read_bool()?;
    let optional_command_support = reader.read_bool()?;
    let mut serial: u16 = reader.read_u8(3)? as u16;

    if serial != 0 {
        serial = 48 + 16 * (serial - 1);
    }

    Ok(XTIDHeader {
        extended_header: extended_header,
        user_memory_permalock: user_memory_permalock,
        blockwrite_blockerase: blockwrite_blockerase,
        optional_command_support: optional_command_support,
        serial_size: serial,
    })
}

/// Look up a mask designer ID and return a string of the manufacturer name
///
/// These mappings are from the [listing on the GS1
/// website](https://www.gs1.org/epcglobal/standards/mdid).
pub fn mdid_name(mdid: &u16) -> &str {
    // These are all binary because that's how they are on the website, for some ridiculous reason.
    match mdid {
        0b000000001 => "Impinj",
        0b000000010 => "Texas Instruments",
        0b000000011 => "Alien Technology",
        0b000000100 => "Intelleflex",
        0b000000101 => "Atmel",
        0b000000110 => "NXP Semiconductors",
        0b000000111 => "ST Microelectronics",
        0b000001000 => "EP Microelectronics",
        0b000001001 => "Motorola (formerly Symbol Technologies)",
        0b000001010 => "Sentech Snd Bhd",
        0b000001011 => "EM Microelectronics",
        0b000001100 => "Renesas Technology Corp.",
        0b000001101 => "Mstar",
        0b000001110 => "Tyco International",
        0b000001111 => "Quanray Electronics",
        0b000010000 => "Fujitsu",
        0b000010001 => "LSIS",
        0b000010010 => "CAEN RFID srl",
        0b000010011 => "Productivity Engineering GmbH",
        0b000010100 => "Federal Electric Corp.",
        0b000010101 => "ON Semiconductor",
        0b000010110 => "Ramtron",
        0b000010111 => "Tego",
        0b000011000 => "Ceitec S.A.",
        0b000011001 => "CPA Wernher von Braun",
        0b000011010 => "TransCore",
        0b000011011 => "Nationz",
        0b000011100 => "Invengo",
        0b000011101 => "Kiloway",
        0b000011110 => "Longjing Microelectronics Co. Ltd.",
        0b000011111 => "Chipus Microelectronics",
        0b000100000 => "ORIDAO",
        0b000100001 => "Maintag",
        0b000100010 => "Yangzhou Daoyuan Microelectronics Co. Ltd",
        0b000100011 => "Gate Elektronik",
        0b000100100 => "RFMicron, Inc.",
        0b000100101 => "RST-Invent LLC",
        0b000100110 => "Crystone Technology",
        0b000100111 => "Shanghai Fudan Microelectronics Group ",
        0b000101000 => "Farsens",
        0b000101001 => "Giesecke & Devrient GmbH",
        0b000101010 => "AWID",
        0b000101011 => "Unitec Semicondutores S/A",
        0b000101100 => "Q-Free ASA",
        0b000101101 => "Valid S.A.",
        0b000101110 => "Fraunhofer IPMS",
        0b000101111 => "ams AG",
        0b000110000 => "Angstrem JSC",
        0b000110001 => "Honeywell",
        0b000110010 => "Huada Semiconductor Co. Ltd (HDSC)",
        0b000110011 => "Lapis Semiconductor Co., Ltd.",
        0b000110100 => "PJSC Mikron",
        0b000110101 => "Hangzhou Landa Microelectronics Co., Ltd.",
        0b000110110 => "Nanjing NARI Micro-Electronic Technology Co., Ltd.",
        0b000110111 => "Southwest Integrated Circuit Design Co., Ltd.",
        0b000111000 => "Silictec",
        0b000111001 => "Nation RFID",
        0b000111010 => "Asygn",
        0b000111011 => "Suzhou HCTech Technology Co., Ltd.",
        0b000111100 => "AXEM Technology",
        _unknown => "Unknown",
    }
}


/// Look up the model name of a tag given the MDID and TMID.
///
/// This data has been extracted from various datasheets - it's definitely not complete and it may
/// not be correct.
pub fn tmid_name(mdid: &u16, tmid: &u16) -> &'static str {
    match (mdid, tmid) {
        // Impinj
        (0x1, 0x100) => "Monza 4D",
        (0x1, 0x105) => "Monza 4QT",
        (0x1, 0x10C) => "Monza 4E",
        (0x1, 0x130) => "Monza 5",
        (0x1, 0x160) => "Monza R6",
        // Alien
        (0x3, 0x412) => "Higgs-3",
        (0x3, 0x414) => "Higgs-4",
        // NXP
        (0x6, 0x003) => "UCODE G2XM",
        (0x6, 0x004) => "UCODE G2XL",
        (0x6, 0x806) => "UCODE G2iL",
        (0x6, 0x807) => "UCODE G2iL+",
        (0x6, 0x80A) => "UCODE G2iM",
        (0x6, 0x80D) => "UCODE i2c",
        (0x6, 0x88D) => "UCODE i2c",
        (0x6, 0x810) => "UCODE 7",
        (0x6, 0x890) => "UCODE 7",
        (0x6, 0x891) => "UCODE 7m",
        (0x6, 0x894) => "UCODE 8",
        (0x6, 0x906) => "UCODE G2iL",
        (0x6, 0x907) => "UCODE G2iL+",
        (0x6, 0x994) => "UCODE 8m",
        (0x6, 0xB06) => "UCODE G2iL",
        (0x6, 0xB07) => "UCODE G2iL+",
        // RFMicron
        (0x24, 0x401) => "Magnus S2",
        (0x24, 0x402) => "Magnus S2",
        (0x24, 0x403) => "Magnus S2",
        _unknown => "Unknown"
    }
}
