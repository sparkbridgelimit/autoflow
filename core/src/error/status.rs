use super::macros::status_codes;

use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::num::NonZeroU16;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatusCode(NonZeroU16);

pub struct InvalidStatusCode {
    _priv: (),
}

impl StatusCode {
    pub fn from_bytes(src: &[u8]) -> Result<StatusCode, InvalidStatusCode> {
        if src.len() != 3 {
            return Err(InvalidStatusCode::new());
        }

        let a = src[0].wrapping_sub(b'0') as u16;
        let b = src[1].wrapping_sub(b'0') as u16;
        let c = src[2].wrapping_sub(b'0') as u16;

        if a == 0 || a > 9 || b > 9 || c > 9 {
            return Err(InvalidStatusCode::new());
        }

        let status = (a * 100) + (b * 10) + c;
        NonZeroU16::new(status)
            .map(StatusCode)
            .ok_or_else(InvalidStatusCode::new)
    }

    pub fn canonical_reason(&self) -> Option<&'static str> {
        canonical_reason(self.0.get())
    }

    #[inline]
    pub fn as_u16(&self) -> u16 {
        (*self).into()
    }

    #[inline]
    pub fn from_u16(src: u16) -> Result<StatusCode, InvalidStatusCode> {
        if src < 100 || src >= 1000 {
            return Err(InvalidStatusCode::new());
        }

        NonZeroU16::new(src)
            .map(StatusCode)
            .ok_or_else(InvalidStatusCode::new)
    }
}

impl fmt::Debug for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            u16::from(*self),
            self.canonical_reason().unwrap_or("<unknown status code>")
        )
    }
}

impl Default for StatusCode {
    #[inline]
    fn default() -> StatusCode {
        StatusCode::OK
    }
}

impl PartialEq<u16> for StatusCode {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        self.as_u16() == *other
    }
}

impl PartialEq<StatusCode> for u16 {
    #[inline]
    fn eq(&self, other: &StatusCode) -> bool {
        *self == other.as_u16()
    }
}

impl From<StatusCode> for u16 {
    #[inline]
    fn from(status: StatusCode) -> u16 {
        status.0.get()
    }
}

impl FromStr for StatusCode {
    type Err = InvalidStatusCode;

    fn from_str(s: &str) -> Result<StatusCode, InvalidStatusCode> {
        StatusCode::from_bytes(s.as_ref())
    }
}

impl<'a> From<&'a StatusCode> for StatusCode {
    #[inline]
    fn from(t: &'a StatusCode) -> Self {
        t.clone()
    }
}

impl<'a> TryFrom<&'a [u8]> for StatusCode {
    type Error = InvalidStatusCode;

    #[inline]
    fn try_from(t: &'a [u8]) -> Result<Self, Self::Error> {
        StatusCode::from_bytes(t)
    }
}

impl<'a> TryFrom<&'a str> for StatusCode {
    type Error = InvalidStatusCode;

    #[inline]
    fn try_from(t: &'a str) -> Result<Self, Self::Error> {
        t.parse()
    }
}

impl TryFrom<u16> for StatusCode {
    type Error = InvalidStatusCode;

    #[inline]
    fn try_from(t: u16) -> Result<Self, Self::Error> {
        StatusCode::from_u16(t)
    }
}

status_codes! {
  (200, OK, "OK");
  (500, INTERNAL_SERVER_ERROR, "Internal Server Error");
}

impl InvalidStatusCode {
    fn new() -> InvalidStatusCode {
        InvalidStatusCode { _priv: () }
    }
}

impl fmt::Debug for InvalidStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InvalidStatusCode")
            // skip _priv noise
            .finish()
    }
}

impl fmt::Display for InvalidStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid status code")
    }
}

impl Error for InvalidStatusCode {}
