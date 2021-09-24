#[macro_use] extern crate lazy_static;
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
    #[error("numerical type value out of range (>=64): {0}")]
    InvalidTypeNumber(u8),
    #[error("type name begins with invalid letter not in [a-zA-Z0-9-_]: {0}")]
    InvalidTypeFirstLetter(String),
    #[error("parent type is non-experimental: {0}")]
    NonExperimentalParentType(u8),
    #[error("invalid non-ascii type name: {0}")]
    InvalidTypeName(String)
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
        63
    }
}

pub fn ch(i: u8) -> char {
    if i < 64 {
        CDE_ALPHABET.as_bytes()[i as usize] as char
    } else {
        '_'
    }
}

pub trait CryptoData {
    fn tag(&self) -> String;
}

mod tag;
pub use tag::*;
