use crate::CdeError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct SubSubClass(u8);

impl SubSubClass {
    pub fn new(val: u8) -> Self {
        SubSubClass(val)
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

impl<T: AsRef<[u8]>> From<T> for SubSubClass {
    fn from(v: T) -> Self {
        SubSubClass(v.as_ref()[1] & 0x07)
    }
}

impl Default for SubSubClass {
    fn default() -> Self {
        SubSubClass(0)
    }
}

impl FromStr for SubSubClass {
    type Err = CdeError;
    fn from_str(tag: &str) -> Result<Self, Self::Err> {
        if let Ok(val) = tag.parse::<u8>() {
            if val < 8 {
                return Ok(SubSubClass(val));
            }
        }
        Err(CdeError::InvalidSubSubClass(tag.to_string()))
    }
}


