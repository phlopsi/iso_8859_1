use crate::string::String;
use ::core::convert::AsRef;
use ::core::convert::From;
use ::core::ops::Add;
use ::core::ops::Deref;
use ::std::borrow::ToOwned;
use ::std::vec::Vec;

#[repr(transparent)]
#[derive(PartialEq, Eq, Hash, Ord, PartialOrd, Clone)]
pub struct StringBuffer(Vec<u8>);

impl StringBuffer {
    #[inline]
    pub fn push_string(&mut self, string: &String) {
        self.0.extend_from_slice(string.as_bytes())
    }

    #[inline]
    pub unsafe fn from_iso_8859_1_unchecked(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    #[inline]
    pub fn as_string(&self) -> &String {
        self
    }
}

impl Add<&String> for StringBuffer {
    type Output = Self;

    #[inline]
    fn add(mut self, other: &String) -> Self {
        self.push_string(other);
        self
    }
}

impl From<&String> for StringBuffer {
    #[inline]
    fn from(s: &String) -> Self {
        unsafe { Self::from_iso_8859_1_unchecked(s.as_bytes().to_owned()) }
    }
}

impl AsRef<[u8]> for StringBuffer {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Deref for StringBuffer {
    type Target = String;

    #[inline]
    fn deref(&self) -> &String {
        String::from_bytes(&self.0)
    }
}
