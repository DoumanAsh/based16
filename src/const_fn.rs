use core::mem;

use crate::{CharPair, CharTable, CHAR_TABLE_LOWER, CHAR_TABLE_UPPER, DecodeError};

#[inline(always)]
pub(crate) const fn dec2hex(table: CharTable, byt: u8) -> CharPair {
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
pub const fn const_hex_upper<const N: usize>(input: [u8; N]) -> [CharPair; N] {
    hex(CHAR_TABLE_UPPER, input)
}

#[inline(always)]
///Creates HEX encoded array out of input
pub const fn const_hex_lower<const N: usize>(input: [u8; N]) -> [CharPair; N] {
    hex(CHAR_TABLE_LOWER, input)
}

#[cold]
#[inline(never)]
const fn unexpected_char(ch: u8) -> DecodeError {
    DecodeError::InvalidChar(ch)
}

#[inline(always)]
pub(crate) const fn hex2dec(ch: u8) -> Result<u8, DecodeError> {
    match ch {
        b'A'..=b'F' => Ok(ch - b'A' + 10),
        b'a'..=b'f' => Ok(ch - b'a' + 10),
        b'0'..=b'9' => Ok(ch - b'0'),
        ch => Err(unexpected_char(ch)),
    }
}
