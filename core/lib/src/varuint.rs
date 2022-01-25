use core::{convert::{From, Into}, fmt, ops::{Deref, DerefMut}};
use crate::{CryptoData, ENCODER};

#[derive(Clone, Copy, Default, PartialEq)]
pub struct VarUInt(u64);

impl<'a> From<&'a [u8]> for VarUInt {
    fn from(buf: &'a [u8]) -> VarUInt {
        let mut v = 0u64;
        for (i, b) in buf.iter().cloned().enumerate() {
            let k = u64::from(b & 0x7f);
            v |= k << (i * 7);
            if b & 0x80 == 0 {
                break;
            }
        }
        VarUInt(v)
    }
}

impl From<u64> for VarUInt {
    fn from(v: u64) -> VarUInt {
        VarUInt(v)
    }
}

impl Into<u64> for VarUInt {
    fn into(self) -> u64 {
        self.0
    }
}

impl From<usize> for VarUInt {
    fn from(v: usize) -> VarUInt {
        VarUInt(v as u64)
    }
}

impl Into<usize> for VarUInt {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl fmt::Display for VarUInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for VarUInt {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for VarUInt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl CryptoData for VarUInt {
    fn len(&self) -> usize {
        match self.0 {
            n if n < 128 => 1,
            n if n >= 128 && n < 268_435_456 => 4,
            n if n >= 268_435_456 && n < 562_949_953_421_311 => 7,
            _ => 7
        }
    }

    fn bytes(&self, buf: &mut [u8]) -> usize {
        let mut v = self.0;
        for b in buf.iter_mut() {
            *b = v as u8 | 0x80;
            v >>= 7;
            if v == 0 {
                *b &= 0x7f;
                break;
            }
        }
        self.len()
    }

    fn encode_len(&self) -> usize {
        ENCODER.encode_len(self.len())
    }

    fn encode(&self, buf: &mut [u8]) -> usize {
        let mut b = [0u8;7];
        self.bytes(&mut b);
        ENCODER.encode_mut(&b[0..self.len()], buf);
        self.encode_len()
    }
}


