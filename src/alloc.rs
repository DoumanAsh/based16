extern crate alloc;

use crate::{DecodeError, CharTable, CHAR_TABLE_UPPER, CHAR_TABLE_LOWER};
use crate::{arch, required_encode_len, required_decode_len};

use alloc::vec::Vec;

fn hex_to_vec(table: CharTable, input: &[u8], out: &mut Vec<u8>) -> usize {
    let required_len = required_encode_len(input.len());
    out.reserve(required_len);

    let result = arch::hex(table, input, out.spare_capacity_mut());
    unsafe {
        out.set_len(out.len() + required_len);
    }

    result
}

#[inline(always)]
///Writes upper case hex appending to `out` vector
pub fn hex_upper_to_vec(input: &[u8], out: &mut Vec<u8>) -> usize {
    hex_to_vec(CHAR_TABLE_UPPER, input, out)
}

#[inline(always)]
///Writes lower case hex appending to `out` vector
pub fn hex_lower_to_vec(input: &[u8], out: &mut Vec<u8>) -> usize {
    hex_to_vec(CHAR_TABLE_LOWER, input, out)
}

#[inline(always)]
///Decodes hex-encoded `input` into `out`, truncating by its size, if necessary.
///
///On error, vector length remains unchanged, but capacity may be changed.
pub fn unhex_to_vec(input: &[u8], out: &mut Vec<u8>) -> Result<usize, DecodeError> {
    let required_len = required_decode_len(input.len());
    out.reserve(required_len);

    let result = arch::unhex(input, out.spare_capacity_mut())?;
    unsafe {
        out.set_len(out.len() + required_len);
    }

    Ok(result)
}
