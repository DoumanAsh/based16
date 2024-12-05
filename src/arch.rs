use core::{cmp, mem};

use crate::{CharTable, DecodeError};
use crate::const_fn::unhex_pair;
use crate::{required_encode_len, required_decode_len};

const CHUNK_LEN: usize = 16;

#[cfg(not(target_feature = "sse2"))]
pub fn hex(table: CharTable, input: &[u8], output: &mut [mem::MaybeUninit<u8>]) -> usize {
    let len = if output.len() % 2 != 0 {
        cmp::min(required_encode_len(input.len()), output.len() - 1)
    } else {
        cmp::min(required_encode_len(input.len()), output.len())
    };

    let mut cursor = input.as_ptr();
    let mut written = 0;

    macro_rules! process_byte {
        ($offset:expr) => {
            unsafe {
                let byt = cursor.add($offset / 2).read();
                let dst = output.as_mut_ptr().add(written + $offset) as *mut u8;

                *dst = table[(byt.wrapping_shr(4) & 0xf) as usize];
                *dst.add(1) = table[(byt & 0xf) as usize];
            }
        };
    }

    macro_rules! on_proceess_end {
        ($chunk_size:expr) => {
            unsafe {
                cursor = cursor.add($chunk_size / 2);
            }
            written = written.saturating_add($chunk_size);
        };
    }

    //We write CHUNKLEN out of CHUNK_LEN / 2
    for _ in 0..len / CHUNK_LEN {
        process_byte!(0);
        process_byte!(2);
        process_byte!(4);
        process_byte!(6);
        process_byte!(8);
        process_byte!(10);
        process_byte!(12);
        process_byte!(14);

        on_proceess_end!(CHUNK_LEN);
    }

    while written < len {
        process_byte!(0);

        on_proceess_end!(2);
    }

    written
}

#[cfg(target_feature = "sse2")]
pub fn hex(table: CharTable, input: &[u8], output: &mut [mem::MaybeUninit<u8>]) -> usize {
    #[cfg(target_arch = "x86")]
    use core::arch::x86 as sys;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64 as sys;

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

#[cfg(target_feature = "sse2")]
pub fn unhex(input: &[u8], output: &mut [mem::MaybeUninit<u8>]) -> Result<usize, DecodeError> {
    const OUTPUT_CHUNK: usize = CHUNK_LEN / 2;

    #[cfg(target_arch = "x86")]
    use core::arch::x86 as sys;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64 as sys;

    let len = cmp::min(required_decode_len(input.len()), output.len());

    let mut simd_len = len / CHUNK_LEN;
    let mut cursor = 0usize;

    //Reference: http://0x80.pl/notesen/2022-01-17-validating-hex-parse.html
    if simd_len > 0 {
        loop {
            unsafe {
                let chunk = sys::_mm_loadu_si128(input.as_ptr().add(cursor.saturating_mul(2)) as _);

                let mut t1 = sys::_mm_add_epi8(chunk, sys::_mm_set1_epi8((0xff - b'9') as i8));
                let mut t2 = sys::_mm_subs_epu8(t1, sys::_mm_set1_epi8(6));
                let t3 = sys::_mm_sub_epi8(t2, sys::_mm_set1_epi8(0xf0u8 as i8));
                let t4 = sys::_mm_and_si128(chunk, sys::_mm_set1_epi8(0xdfu8 as i8));
                let t5 = sys::_mm_sub_epi8(t4, sys::_mm_set1_epi8(b'A' as i8));
                let t6 = sys::_mm_adds_epu8(t5, sys::_mm_set1_epi8(10));

                let nibbles = sys::_mm_min_epu8(t3, t6);
                let t8 = sys::_mm_adds_epu8(nibbles, sys::_mm_set1_epi8(127-15));

                if sys::_mm_movemask_epi8(t8) > 0 {
                    return Err(DecodeError::unexpected_char(input[0]));
                }

                //convert to actual binary output
                let result = {
                    let low = sys::_mm_srli_epi16(nibbles, 8);
                    let high = sys::_mm_slli_epi16(nibbles, 4);
                    t1 = sys::_mm_or_si128(low, high);
                    t2 = sys::_mm_and_si128(t1, sys::_mm_set1_epi16(0x00ff));
                    let t3: [u64; 2] = core::mem::transmute(sys::_mm_packus_epi16(t2, sys::_mm_setzero_si128()));
                    t3[0]
                };

                core::ptr::write_unaligned(output.as_mut_ptr().add(cursor) as *mut u64, result)
            }

            cursor = cursor.saturating_add(OUTPUT_CHUNK);
            if simd_len == 0 {
                break;
            } else {
                simd_len = simd_len.saturating_sub(1);
            }
        }
    }

    while cursor < len {
        let chunk = unsafe {
            *(input.as_ptr().add(cursor.saturating_mul(2)) as *const [u8; 2])
        };
        let char = unhex_pair(chunk)?;
        output[cursor] = mem::MaybeUninit::new(char);
        cursor = cursor.saturating_add(1);
    }

    Ok(cursor)
}

#[cfg(not(target_feature = "sse2"))]
pub fn unhex(input: &[u8], output: &mut [mem::MaybeUninit<u8>]) -> Result<usize, DecodeError> {
    let len = cmp::min(required_decode_len(input.len()), output.len());

    let mut cursor = 0usize;
    let mut written = 0usize;
    macro_rules! process_byte {
        ($offset:expr) => {
            unsafe {
                let chunk = unsafe {
                    *(input.as_ptr().add(cursor + $offset) as *const [u8; 2])
                };
                let ch = unhex_pair(chunk)?;
                *output.get_unchecked_mut(written + ($offset / 2)) = mem::MaybeUninit::new(ch);
            }
        };
    }

    macro_rules! on_proceess_end {
        ($chunk_size:expr) => {
            written = written.saturating_add($chunk_size / 2);
            cursor = cursor.wrapping_add($chunk_size)
        };
    }

    // We decode CHUNK_LEN / 2 out of CHUNK_LEN
    for _ in 0..len / CHUNK_LEN {
        process_byte!(0);
        process_byte!(2);
        process_byte!(4);
        process_byte!(6);
        process_byte!(8);
        process_byte!(10);
        process_byte!(12);
        process_byte!(14);

        on_proceess_end!(CHUNK_LEN);
    }

    while written < len {
        process_byte!(0);

        on_proceess_end!(2);
    }

    Ok(written)
}
