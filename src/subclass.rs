use crate::{
    idx,
    CdeError
};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct SubClass(u8);

impl SubClass {
    pub fn new(val: u8) -> Self {
        SubClass(val)
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

impl<T: AsRef<[u8]>> From<T> for SubClass {
    fn from(v: T) -> Self {
        SubClass((((v.as_ref()[0] & 0x03) << 4) | ((v.as_ref()[1] & 0xF0) >> 4)) & 0x3F)
    }
}

impl FromStr for SubClass {
    type Err = CdeError;
    fn from_str(tag: &str) -> Result<Self, Self::Err> {
        match tag.to_lowercase().as_str() {
            "m" | "o" | "p" | "x" | "_" => {
                if let Some(c) = tag.chars().next() {
                    Ok(SubClass(idx(c)))
                } else {
                    Err(CdeError::InvalidSubClass(tag.to_string()))
                }
            }
            "minisign" => Ok(SubClass(idx('m'))),
            "openssl" => Ok(SubClass(idx('o'))),
            "openpgp" => Ok(SubClass(idx('p'))),
            "x509" => Ok(SubClass(idx('x'))),
            "nontyped" => Ok(SubClass(idx('_'))),
            _ => Err(CdeError::InvalidSubClass(tag.to_string()))
        }
    }
}

impl Default for SubClass {
    fn default() -> Self {
        SubClass(idx('_'))
    }
}

impl Into<u8> for SubClass {
    fn into(self) -> u8 {
        self.0
    }
}


