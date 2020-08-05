use ::core::convert::AsRef;
use ::core::convert::From;
use ::core::convert::Into;
use ::core::fmt;
use ::core::hint::unreachable_unchecked;
use ::core::result::Result;
use ::encoding::all::ISO_8859_1;
use ::encoding::types::Encoding;
use ::encoding::DecoderTrap;
use ::serde::de;
use ::std::boxed::Box;

#[repr(transparent)]
#[derive(PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct String([u8]);

impl String {
    #[inline(always)]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    #[inline(always)]
    pub const fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    #[inline(always)]
    pub const fn from_bytes(f: &[u8]) -> &Self {
        #[repr(C)]
        union Slices<'a> {
            byte_slice: &'a [u8],
            string: &'a String,
        }

        unsafe { Slices { byte_slice: f }.string }
    }

    #[inline(always)]
    pub const fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl fmt::Display for String {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(
            &Encoding::decode(ISO_8859_1, &self.0, DecoderTrap::Strict)
                .unwrap_or_else(|_| unsafe { unreachable_unchecked() }),
            f,
        )
    }
}

impl fmt::Debug for String {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(
            &Encoding::decode(ISO_8859_1, &self.0, DecoderTrap::Strict)
                .unwrap_or_else(|_| unsafe { unreachable_unchecked() }),
            f,
        )
    }
}

impl<'a> From<&'a [u8]> for &'a String {
    #[inline(always)]
    fn from(f: &'a [u8]) -> Self {
        String::from_bytes(f)
    }
}

impl<'a> From<&'a String> for &'a [u8] {
    #[inline(always)]
    fn from(f: &'a String) -> Self {
        f.as_bytes()
    }
}

impl<'de: 'a, 'a> de::Deserialize<'de> for &'a String {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct StringVisitor;

        impl<'de> de::Visitor<'de> for StringVisitor {
            type Value = &'de String;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a borrowed ISO 8859-1 string")
            }

            #[inline]
            fn visit_borrowed_bytes<E>(self, v: &'de [u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Result::Ok(Into::into(v))
            }
        }

        deserializer.deserialize_bytes(StringVisitor)
    }
}

impl From<&String> for Box<String> {
    #[inline]
    fn from(s: &String) -> Box<String> {
        unsafe { from_boxed_iso_8859_1_unchecked(Box::from(Into::<&[u8]>::into(s))) }
    }
}

impl AsRef<[u8]> for String {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.into()
    }
}

pub unsafe fn from_boxed_iso_8859_1_unchecked(v: Box<[u8]>) -> Box<String> {
    Box::from_raw(Box::into_raw(v) as *mut String)
}
