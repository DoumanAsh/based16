//!Simple Base-16 (aka hexidecimal) encoding

#![no_std]
#![warn(missing_docs)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

use core::{fmt, mem};

mod pair;
pub use pair::CharPair;
mod arch;
mod const_fn;
pub use const_fn::*;
#[cfg(feature = "alloc")]
mod alloc;
#[cfg(feature = "alloc")]
pub use alloc::*;

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
///Writes upper case hex into `out`
pub fn hex_upper(input: &[u8], out: &mut [mem::MaybeUninit<u8>]) -> usize {
    arch::hex(CHAR_TABLE_UPPER, input, out)
}

#[inline(always)]
///Writes lower case hex into `out`
pub fn hex_lower(input: &[u8], out: &mut [mem::MaybeUninit<u8>]) -> usize {
    arch::hex(CHAR_TABLE_LOWER, input, out)
}

#[inline(always)]
///Decodes hex-encoded `input` into `out`, truncating by its size, if necessary.
pub fn unhex(input: &[u8], out: &mut [mem::MaybeUninit<u8>]) -> Result<usize, DecodeError> {
    arch::unhex(input, out)
}

#[derive(Debug, Copy, Clone)]
///Error happening during decoding
pub enum DecodeError {
    ///Invalid character encountered
    InvalidChar(u8)
}

impl DecodeError {
    #[cold]
    #[inline(never)]
    pub(crate) const fn unexpected_char(ch: u8) -> Self {
        Self::InvalidChar(ch)
    }
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

impl Iterator for Encoder<'_> {
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

impl ExactSizeIterator for Encoder<'_> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.data.len()
    }
}

impl fmt::Display for Encoder<'_> {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byt in self.data {
            fmt.write_str(dec2hex(self.table, *byt).as_str())?;
        }

        Ok(())
    }
}

impl fmt::Debug for Encoder<'_> {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str("\"")?;
        for byt in self.data {
            fmt.write_str(dec2hex(self.table, *byt).as_str())?;
        }

        fmt.write_str("\"")
    }
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
        unhex_pair(chunk)
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

impl Iterator for Decoder<'_> {
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

impl ExactSizeIterator for Decoder<'_> {
    #[inline(always)]
    fn len(&self) -> usize {
        self.0.len()
    }
}
