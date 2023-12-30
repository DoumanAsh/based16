use core::{cmp, mem};

use crate::{CharTable, DecodeError};
use crate::const_fn::hex2dec;

use crate::{required_encode_len, required_decode_len};

#[cfg(not(target_feature = "sse2"))]
pub fn hex(table: CharTable, input: &[u8], output: &mut [mem::MaybeUninit<u8>]) -> usize {
    let len = if output.len() % 2 != 0 {
        cmp::min(required_encode_len(input.len()), output.len() - 1)
    } else {
        cmp::min(required_encode_len(input.len()), output.len())
    };

    let mut written = 0;
    while written < len {
        unsafe {
            let byt = *(input.as_ptr().add(written / 2));
            let dst = output.as_mut_ptr().add(written) as *mut u8;

            *dst = table[(byt.wrapping_shr(4) & 0xf) as usize];
            *dst.add(1) = table[(byt & 0xf) as usize];
        }

        written = written.saturating_add(2);
    }

    written
}

#[cfg(target_feature = "sse2")]
pub fn hex(table: CharTable, input: &[u8], output: &mut [mem::MaybeUninit<u8>]) -> usize {
    #[cfg(target_arch = "x86")]
    use core::arch::x86 as sys;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64 as sys;

    const CHUNK_LEN: usize = 16;
    let len = if output.len() % 2 != 0 {
        cmp::min(required_encode_len(input.len()), output.len() - 1)
    } else {
        cmp::min(required_encode_len(input.len()), output.len())
    };
    let mut written = 0;

    if len >= CHUNK_LEN {
        unsafe {
            let mask = sys::_mm_set1_epi8(0xf);
            let mask1 = sys::_mm_set1_epi8(0x9);
            let mask2 = sys::_mm_set1_epi8((*table.get_unchecked(10) - *table.get_unchecked(0) - 0xA) as _);
            let mask3 = sys::_mm_set1_epi8(*table.get_unchecked(0) as _);

            loop {
                let mut value = sys::_mm_loadu_si64(input.as_ptr().add(written / 2));
                value = sys::_mm_and_si128(sys::_mm_unpacklo_epi8(sys::_mm_srli_epi64(value, 4), value), mask);
                value = sys::_mm_add_epi8(
                    sys::_mm_add_epi8(value, mask3),
                    sys::_mm_and_si128(sys::_mm_cmpgt_epi8(value, mask1), mask2)
                );
                sys::_mm_storeu_si128(output.as_mut_ptr().add(written) as _, value);
                written = written.saturating_add(CHUNK_LEN);

                if (len - written) < CHUNK_LEN {
                    break;
                }
            }
        }
    }

    while written < len {
        unsafe {
            let byt = *(input.as_ptr().add(written / 2));
            let dst = output.as_mut_ptr().add(written) as *mut u8;

            *dst = table[(byt.wrapping_shr(4) & 0xf) as usize];
            *dst.add(1) = table[(byt & 0xf) as usize];
        }

        written = written.saturating_add(2);
    }

    written
}

//#[cfg(not(target_feature = "sse2"))]
pub fn unhex(input: &[u8], output: &mut [mem::MaybeUninit<u8>]) -> Result<usize, DecodeError> {
    const INVALID_CHAR: u8 = 0xff;
    const UNHEX_TABLE: &[u8; 256] = &{
        let mut res = [0u8; 256];
        let mut idx = 0usize;
        while idx <= (u8::MAX as usize) {
            res[idx] = match hex2dec(idx as u8) {
                Ok(res) => res,
                Err(_) => INVALID_CHAR,
            };
            idx += 1;
        }
        res
    };

    let len = cmp::min(required_decode_len(input.len()), output.len());

    let mut written = 0usize;
    while written < len {
        unsafe {
            let left = UNHEX_TABLE.get_unchecked(*input.get_unchecked(written * 2) as usize);
            let right = UNHEX_TABLE.get_unchecked(*input.get_unchecked(written * 2 + 1) as usize);

            *output.get_unchecked_mut(written) = mem::MaybeUninit::new(left.wrapping_shl(4) | right);

            if (left | right) == INVALID_CHAR {
                return Err(DecodeError::InvalidChar(*left));
            }
        }

        written = written.saturating_add(1);
    }

    Ok(written)
}
