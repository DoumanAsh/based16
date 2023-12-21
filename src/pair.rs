use core::{fmt, ops, mem};

///Character pair representing single byte
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
#[repr(transparent)]
pub struct CharPair(pub(crate) [u8; 2]);

impl CharPair {
    #[inline(always)]
    ///Returns pair of chars
    pub const fn as_chars(&self) -> [char; 2] {
        [
            self.0[0] as char,
            self.0[1] as char,
        ]
    }

    #[inline(always)]
    ///Returns pair of chars as string
    pub const fn as_bytes(&self) -> &[u8; 2] {
        &self.0
    }

    #[inline(always)]
    ///Returns pair of chars as string
    pub const fn as_str(&self) -> &'_ str {
        unsafe {
            core::str::from_utf8_unchecked(&self.0)
        }
    }

    #[inline(always)]
    ///Converts array of pairs into byte slice
    pub const fn array_as_bytes<const N: usize>(array: &[Self; N]) -> &[u8] {
        unsafe {
            core::slice::from_raw_parts(array.as_ptr() as *const u8, N * mem::size_of::<Self>())
        }
    }

    #[inline(always)]
    ///Converts array of pairs into string
    pub const fn array_as_str<const N: usize>(array: &[Self; N]) -> &'_ str {
        unsafe {
            core::str::from_utf8_unchecked(
                Self::array_as_bytes(array)
            )
        }
    }
}

impl fmt::Debug for CharPair {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), fmt)
    }
}

impl fmt::Display for CharPair {
    #[inline(always)]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), fmt)
    }
}

impl ops::Deref for CharPair {
    type Target = str;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AsRef<str> for CharPair {
    #[inline(always)]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<[u8]> for CharPair {
    #[inline(always)]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsRef<[u8; 2]> for CharPair {
    #[inline(always)]
    fn as_ref(&self) -> &[u8; 2] {
        &self.0
    }
}
