//! Parser for the .rcx binary format
//!
//! Referenced from: <https://github.com/BrickBot/nqc/blob/master/rcxlib/RCX_Image.cpp>
//!
//! ```text
//! * signature - 4 bytes
//! * version - 2 bytes
//! * chunks_count - 2 bytes
//! * symbol_count - 2 bytes
//! * target_type - 1 byte
//! * reserved - 1 byte
//! * for each chunk:
//!   - type - 1 byte (type <= 2)
//!   - number - 1 byte
//!   - length - 2 bytes
//!   - data - <length> bytes
//!  * for each symbol:
//!   - type - 1 byte
//!   - index - 1 byte
//!   - length - 1 byte
//!   - name - <length> bytes cstr
//! ```

use crate::Result;
use color_eyre::eyre::eyre;
use nom::{number::Endianness, IResult};
use std::{
    ffi::CString,
    fmt::{self, Debug, Display, Formatter},
};

const RCX_TAG: &str = "RCXI";
const MAX_SECTIONS: usize = 10;
const INDENT: &str = "  ";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RcxBin {
    pub signature: [u8; 4],
    pub version: u16,
    pub section_count: u16,
    pub symbol_count: u16,
    pub target_type: u8,
    pub reserved: u8,
    pub sections: Vec<Section>,
    pub symbols: Vec<Symbol>,
}

impl RcxBin {
    pub fn parse(bin: &[u8]) -> Result<Self> {
        let (_i, bin) = parse(bin).map_err(|e| eyre!("Parse error: {e}"))?;
        bin.verify()?;
        Ok(bin)
    }

    pub fn verify(&self) -> Result<()> {
        fn repeated_idx(sections: &[Section]) -> bool {
            let mut c = sections.iter().map(|c| c.number).collect::<Vec<_>>();
            c.sort_unstable();
            c.dedup();
            c.len() != sections.len()
        }

        // check chunk count
        if self.section_count as usize != self.sections.len()
            || self.sections.len() > MAX_SECTIONS
        {
            Err(eyre!("Invalid number of chunks"))
        } else if repeated_idx(&self.sections) {
            Err(eyre!("Nonunique chunk numbers"))
        } else {
            Ok(())
        }
    }
}

impl Display for RcxBin {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        writeln!(
            fmt,
            "Signature: {}",
            String::from_utf8_lossy(&self.signature),
        )?;
        writeln!(fmt, "Version: {:x}", self.version)?;
        writeln!(
            fmt,
            "{} sections, {} symbols",
            self.section_count, self.symbol_count,
        )?;
        writeln!(fmt, "Target: {}", self.target_type)?;
        writeln!(fmt, "Sections:")?;
        for section in &self.sections {
            writeln!(fmt, "{section}")?;
        }
        writeln!(fmt, "Symbols:")?;
        for symbol in &self.symbols {
            writeln!(fmt, "{symbol}")?;
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Section {
    pub ty: SectionType,
    pub number: u8,
    pub length: u16,
    pub data: Vec<u8>,
}

fn parse_chunk(i: &[u8]) -> IResult<&[u8], Section> {
    let read_u16 = nom::number::complete::u16(Endianness::Little);
    let read_u8 = nom::number::complete::u8;

    let (i, ty) = SectionType::parse(i)?;
    let (i, number) = read_u8(i)?;
    let (i, length) = read_u16(i)?;
    let (i, data) = nom::bytes::complete::take(length)(i)?;

    Ok((
        i,
        Section {
            ty,
            number,
            length,
            data: data.to_vec(),
        },
    ))
}

impl Display for Section {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        writeln!(fmt, "{INDENT}{} - {} bytes", self.ty, self.length)?;
        writeln!(fmt, "{INDENT}{INDENT}{}", hex::encode(&self.data))?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SectionType {
    Task = 0,
    SubChunk,
    Sound,
    Animation,
    Count,
}

impl SectionType {
    pub fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, ty) = nom::number::complete::u8(i)?;
        let ty = match ty {
            0 => Self::Task,
            1 => Self::SubChunk,
            2 => Self::Sound,
            3 => Self::Animation,
            4 => Self::Count,
            _ => {
                return Err(nom::Err::Failure(nom::error::Error {
                    input: i,
                    code: nom::error::ErrorKind::Verify,
                }));
            }
        };
        Ok((i, ty))
    }
}

impl Display for SectionType {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, fmt)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Symbol {
    pub ty: u8,
    pub index: u8,
    pub length: u16,
    pub name: CString,
}

fn parse_symbol(i: &[u8]) -> IResult<&[u8], Symbol> {
    let read_u16 = nom::number::complete::u16(Endianness::Little);
    let read_u8 = nom::number::complete::u8;

    let (i, ty) = read_u8(i)?;
    let (i, index) = read_u8(i)?;
    let (i, length) = read_u16(i)?;
    let (i, name) = nom::bytes::complete::take(length)(i)?;

    Ok((
        i,
        Symbol {
            ty,
            index,
            length,
            name: CString::from_vec_with_nul(name.to_vec()).map_err(|_| {
                nom::Err::Failure(nom::error::Error {
                    input: i,
                    code: nom::error::ErrorKind::Alpha,
                })
            })?,
        },
    ))
}

impl Display for Symbol {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        writeln!(
            fmt,
            "{INDENT}{} at {} - {} bytes",
            self.ty, self.index, self.length
        )?;
        writeln!(fmt, "{INDENT}{INDENT}{:?}", self.name)?;
        Ok(())
    }
}

fn parse(bin: &[u8]) -> IResult<&[u8], RcxBin> {
    let read_u16 = nom::number::complete::u16(Endianness::Little);
    let read_u8 = nom::number::complete::u8;

    let (i, signature) = nom::bytes::complete::tag(RCX_TAG)(bin)?;
    let (i, version) = read_u16(i)?;
    let (i, chunk_count) = read_u16(i)?;
    let (i, symbol_count) = read_u16(i)?;
    let (i, target_type) = read_u8(i)?;
    let (i, reserved) = read_u8(i)?;

    let (i, chunks) = nom::multi::count(parse_chunk, chunk_count.into())(i)?;
    let (i, symbols) = nom::multi::count(parse_symbol, symbol_count.into())(i)?;

    IResult::Ok((
        i,
        RcxBin {
            signature: signature.try_into().unwrap_or([0; 4]),
            version,
            section_count: chunk_count,
            symbol_count,
            target_type,
            reserved,
            sections: chunks,
            symbols,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use hex_literal::hex;

    const SAMPLE: &[u8] = &hex!(
        "5243584902010100010000000000 \
        140013070207e18713010232e1812181 \
        430264002141000005006d61696e00"
    );

    #[test]
    fn parse_sample() {
        let bin = RcxBin::parse(SAMPLE).unwrap();
        assert_eq!(
            bin,
            RcxBin {
                signature: *b"RCXI",
                version: 0x0102,
                section_count: 1,
                symbol_count: 1,
                target_type: 0,
                reserved: 0,
                sections: vec![Section {
                    ty: SectionType::Task,
                    number: 0,
                    length: 20,
                    data: vec![
                        0x13, 0x7, 0x2, 0x7, 0xe1, 0x87, 0x13, 0x1, 0x2, 0x32,
                        0xe1, 0x81, 0x21, 0x81, 0x43, 0x2, 0x64, 0x0, 0x21,
                        0x41
                    ]
                }],
                symbols: vec![Symbol {
                    ty: 0,
                    index: 0,
                    length: 5,
                    name: CString::new("main").unwrap(),
                }],
            }
        );
    }
}
