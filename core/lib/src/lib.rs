use anyhow;
use data_encoding::Encoding;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("general error")]
    GeneralError,
    #[error("io error")]
    IoError(#[from] std::io::Error),
    #[error("fmt error")]
    FmtError(#[from] std::fmt::Error),
    #[error("invalid class")]
    InvalidClass,
    #[error("invalid sub-class")]
    InvalidSubClass,
    #[error("invalid sub-sub-class value")]
    InvalidSubSubClass,
    #[error("numerical type value out of range (>=64)")]
    InvalidTypeNumber(u8),
    #[error("type name begins with invalid letter not in [a-zA-Z0-9-_]")]
    InvalidTypeFirstLetter,
    #[error("parent type is non-experimental")]
    NonExperimentalParentType(u8),
    #[error("invalid non-ascii type name")]
    InvalidTypeName,
    #[error("failed to build tag from str")]
    TagFromStr,
    #[error("failed to build tag from bytes")]
    TagFromBytes,
    #[error("decode error")]
    DecodeError,
}

pub type Result<T> = anyhow::Result<T, Error>;

pub static CDE_ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz01234-ABCDEFGHIJKLMNOPQRSTUVWXYZ56789_";
pub static ENCODER: Encoding = data_encoding_macro::new_encoding! {
    symbols: "abcdefghijklmnopqrstuvwxyz01234-ABCDEFGHIJKLMNOPQRSTUVWXYZ56789_",
};

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

/// The #[cde("class.subclass.subsubclass")] macro automatically generates
/// impls of this trait for your cryptographic data object. the object must
/// impl the CryptoData trait below so that the Tag associated with your object
/// will set the data's length at runtime when the Tag is serialized.
pub trait CryptoDataTag<T: CryptoData> {
    fn as_str(&self) -> &str;
    fn as_bytes(&self) -> &[u8];
}

pub trait CryptoData {
    fn get_length(&self) -> usize;
    fn encode(&self, encoded: &mut [u8]);
}

mod tag;
pub use tag::*;
