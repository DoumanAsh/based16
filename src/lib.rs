//!Simple Base-16 (aka hexidecimal) encoding

#![no_std]
#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

use core::{fmt, mem};

mod pair;
pub use pair::CharPair;

type CharTable = &'static [u8; 16];
const CHAR_TABLE_LOWER: CharTable = b"0123456789abcdef";
const CHAR_TABLE_UPPER: CharTable = b"0123456789ABCDEF";

///Length required to HEX encode
pub const fn required_encode_len(len: usize) -> usize {
    len.saturating_mul(2)
}

///Length required to decode HEX
pub const fn required_decode_len(len: usize) -> usize {
    len.saturating_div(2)
}

#[inline(always)]
const fn dec2hex(table: CharTable, byt: u8) -> CharPair {
    let buf = [
        table[(byt.wrapping_shr(4) & 0xf) as usize],
        table[(byt & 0xf) as usize],
    ];
    CharPair(buf)
}

const fn hex<const N: usize>(table: CharTable, input: [u8; N]) -> [CharPair; N] {
    let mut output = [mem::MaybeUninit::uninit(); N];

    let mut idx = 0;

    while idx < N {
        output[idx] = mem::MaybeUninit::new(dec2hex(table, input[idx]));
        idx += 1;
    }

    unsafe {
        mem::transmute_copy(&output)
    }
}

#[inline(always)]
///Creates HEX encoded array out of input
pub const fn hex_upper<const N: usize>(input: [u8; N]) -> [CharPair; N] {
    hex(CHAR_TABLE_UPPER, input)
}

#[inline(always)]
///Creates HEX encoded array out of input
pub const fn hex_lower<const N: usize>(input: [u8; N]) -> [CharPair; N] {
    hex(CHAR_TABLE_LOWER, input)
}

///Hex encoder, implements iterator returning individual byte as pair of characters.
///
///`Display` implementation renders current data without advancing iterator
pub struct Encoder<'a> {
    table: CharTable,
    data: &'a [u8],
}

impl<'a> Encoder<'a> {
    #[inline(always)]
    ///Creates encoder with upper character set
    pub const fn upper(data: &'a [u8]) -> Self {
        Self {
            table: CHAR_TABLE_UPPER,
            data,
        }
    }

    #[inline(always)]
    ///Creates encoder with lower character set
    pub const fn lower(data: &'a [u8]) -> Self {
        Self {
            table: CHAR_TABLE_LOWER,
            data,
        }
    }

    #[inline(always)]
    ///Get next byte encoded
    pub fn next_byte(&mut self) -> Option<CharPair> {
        match self.data.split_first() {
            Some((byt, rest)) => {
                self.data = rest;
                Some(dec2hex(self.table, *byt))
            },
            None => None
        }
    }
}

impl<'a> Iterator for Encoder<'a> {
    type Item = CharPair;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        self.next_byte()
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.data.len(), Some(self.data.len()))
    }
}

impl<'a> ExactSizeIterator for Encoder<'a> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.data.len()
    }
}

impl<'a> fmt::Display for Encoder<'a> {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byt in self.data {
            fmt.write_str(dec2hex(self.table, *byt).as_str())?;
        }

        Ok(())
    }
}

const fn hex2dec(ch: u8) -> Result<u8, DecodeError> {
    match ch {
        b'A'..=b'F' => Ok(ch - b'A' + 10),
        b'a'..=b'f' => Ok(ch - b'a' + 10),
        b'0'..=b'9' => Ok(ch - b'0'),
        ch => Err(DecodeError::InvalidChar(ch)),
    }
}

#[derive(Debug, Copy, Clone)]
///Error happening during decoding
pub enum DecodeError {
    ///Invalid character encountered
    InvalidChar(u8)
}

///Decoder that transforms pairs of characters into individual decimal bytes
pub struct Decoder<'a>(&'a [u8]);

impl<'a> Decoder<'a> {
    #[inline(always)]
    ///Creates new instance validating that input has even length.
    pub const fn new(data: &'a str) -> Option<Self> {
        if data.len() % 2 != 0 {
            None
        } else {
            Some(Self(data.as_bytes()))
        }
    }

    #[inline]
    fn inner_next_byte(&mut self) -> Result<u8, DecodeError> {
        let chunk = unsafe {
            *(self.0.as_ptr() as *const [u8; 2])
        };
        self.0 = &self.0[2..];
        Ok((hex2dec(chunk[0])? << 4) | hex2dec(chunk[1])?)
    }

    #[inline]
    ///Gets next byte, returning error in case of invalid character
    pub fn next_byte(&mut self) -> Option<Result<u8, DecodeError>> {
        if self.0.is_empty() {
            return None;
        }

        Some(self.inner_next_byte())
    }
}

impl<'a> Iterator for Decoder<'a> {
    type Item = Result<u8, DecodeError>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.next_byte()
    }

    #[inline(always)]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.0.len(), Some(self.0.len()))
    }
}

impl<'a> ExactSizeIterator for Decoder<'a> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.0.len()
    }
}
