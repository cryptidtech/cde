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
    #[error("failed to build from str")]
    FromStr,
    #[error("failed to build from bytes")]
    FromBytes,
    #[error("no buffer given")]
    MissingBuf,
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

pub fn decode_tag_and_data<'a, T: From<&'a [u8]>>(encoded: &[u8], buf: &'a mut [u8]) -> Result<(Tag, T)> {
    let len = ENCODER.decode_len(encoded.len()).map_err(|_| Error::DecodeError)?;
    ENCODER.decode_mut(encoded, &mut buf[0..len]).map_err(|_| Error::DecodeError)?;
    let tag = TagBuilder::from_bytes(&buf).build()?;
    let len = tag.len();
    let data_len = tag.get_data_length() as usize;
    let data = T::from(&buf[len..len + data_len]);
    Ok((tag, data))
}

pub fn encode_tag_and_data<'a>(tag: &mut Tag, data: &impl CryptoData, buf: &'a mut [u8]) -> Result<usize> {
    tag.set_data_length(data.len() as u64);
    let tagsize = tag.encode(buf);
    let datasize = data.encode(&mut buf[tagsize..]);
    Ok(tagsize + datasize)
}

pub trait CryptoData {
    fn len(&self) -> usize;
    fn bytes(&self, buf: &mut [u8]) -> usize;
    fn encode_len(&self) -> usize;
    fn encode(&self, buf: &mut [u8]) -> usize;
}

mod tag;
pub use tag::*;
mod varuint;
pub use varuint::*;
