use crate::{
    idx,
    CdeError
};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct Class(u8);

impl Class {
    pub fn new(val: u8) -> Self {
        Class(val)
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

impl<T: AsRef<[u8]>> From<T> for Class {
    fn from(v: T) -> Self {
        Class(((v.as_ref()[0] & 0xFC) >> 2) & 0x3F)
    }
}

impl FromStr for Class {
    type Err = CdeError;
    fn from_str(tag: &str) -> Result<Self, Self::Err> {
        match tag.to_lowercase().as_str() {
            "s" | "_" => {
                if let Some(c) = tag.chars().next() {
                    Ok(Class(idx(c)))
                } else {
                    Err(CdeError::InvalidClass(tag.to_string()))
                }
            },
            "signature" => Ok(Class(idx('s'))),
            "nontyped" => Ok(Class(idx('_'))),
            _ => Err(CdeError::InvalidClass(tag.to_string()))
        }
    }
}

impl Default for Class {
    fn default() -> Self {
        Class(idx('_'))
    }
}

impl Into<u8> for Class {
    fn into(self) -> u8 {
        self.0
    }
}

