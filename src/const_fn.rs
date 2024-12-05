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

#[inline(always)]
const fn hex2dec(ch: u8) -> Result<u8, DecodeError> {
    match ch {
        b'A'..=b'F' => Ok(ch - b'A' + 10),
        b'a'..=b'f' => Ok(ch - b'a' + 10),
        b'0'..=b'9' => Ok(ch - b'0'),
        ch => Err(DecodeError::unexpected_char(ch)),
    }
}

const UNHEX_INVALID_CHAR: u8 = 0xff;
const UNHEX_TABLE: &[u8; 256] = &{
    let mut res = [0u8; 256];
    let mut idx = 0usize;
    while idx <= (u8::MAX as usize) {
        res[idx] = match hex2dec(idx as u8) {
            Ok(res) => res,
            Err(_) => UNHEX_INVALID_CHAR,
        };
        idx += 1;
    }
    res
};

#[inline(always)]
///Converts hex character pair into underlying byte
pub const fn unhex_pair(ch: [u8; 2]) -> Result<u8, DecodeError> {
    let (left, right) = unsafe {
        let table = UNHEX_TABLE.as_ptr();
        //This is always valid because u8::MAX value will always fit table
        (*table.add(ch[0] as usize), *table.add(ch[1] as usize))
    };

    if left == UNHEX_INVALID_CHAR {
        Err(DecodeError::InvalidChar(ch[0]))
    } else if right == UNHEX_INVALID_CHAR {
        Err(DecodeError::InvalidChar(ch[1]))
    } else {
        Ok(left.wrapping_shl(4) | right)
    }
}
