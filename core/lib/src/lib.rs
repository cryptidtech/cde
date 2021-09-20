use anyhow::Result;
use data_encoding::{ Encoding, Specification, SpecificationError };
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CdeError {
    #[error("invalid class: {0}")]
    InvalidClass(String),
    #[error("invalid sub-class: {0}")]
    InvalidSubClass(String),
    #[error("invalid sub-sub-class value: {0}")]
    InvalidSubSubClass(String),
}

pub static CDE_ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz01234-ABCDEFGHIJKLMNOPQRSTUVWXYZ56789_";

pub fn encoder() -> Result<Encoding, SpecificationError> {
    let mut spec = Specification::new();
    spec.symbols.push_str(CDE_ALPHABET);
    spec.encoding()
}

pub fn idx(c: char) -> u8 {
    if let Some(i) = CDE_ALPHABET.find(c) {
        i as u8
    } else {
        0
    }
}

pub trait CdeTag {
    fn class(&self) -> Class;
    fn subclass(&self) -> SubClass;
    fn subsubclass(&self) -> SubSubClass;
}

mod class;
mod subclass;
mod subsubclass;
mod typetag;
// re-export
pub use class::*;
pub use subclass::*;
pub use subsubclass::*;
pub use typetag::*;
