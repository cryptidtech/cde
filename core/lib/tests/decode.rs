use cde::{CryptoData, decode_tag_and_data, ENCODER, TagBuilder};
use std::fmt::{self, Display, Formatter};
//use rand::{thread_rng, Rng};

struct Key([u8; 32]);

impl Default for Key {
    fn default() -> Self {
        Key([0u8; 32])
    }
}

impl AsMut<[u8]> for Key {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl<'a> From<&'a [u8]> for Key {
    fn from(b: &'a [u8]) -> Key {
        let mut key = Key::default();
        let len = key.len();
        key.as_mut().copy_from_slice(&b[0..len]);
        key
    }
}

impl Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for b in self.0 {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}

impl CryptoData for Key {
    fn len(&self) -> usize {
        32
    }
    fn bytes(&self, buf: &mut [u8]) -> usize {
        buf.copy_from_slice(&self.0);
        self.len()
    }
    fn encode_len(&self) -> usize {
        ENCODER.encode_len(self.len())
    }
    fn encode(&self, buf: &mut [u8]) -> usize {
        ENCODER.encode_mut(&self.0, buf);
        self.encode_len()
    }
}

#[test]
fn decode1() {
    // cde encoded tag and 32-byte key data
    let encoded = b"keeA48J-DbADmUdvXrUjJ_r4XkZv4TEtHRVoFS6oQ0AgY5i";

    // local buffer used for decoding the tag and data into
    let mut b = [0u8; 35];

    let (tag, key) = decode_tag_and_data::<Key>(encoded, &mut b).unwrap();
    assert_eq!("key.ed25519.secret", format!("{}", tag));
    assert_eq!("7bda5f8c18233340d5dd1d09a7f45edcae557b39139f1d4e972ecec1a806e3a2", format!("{}", key));
}

#[test]
fn decode2() {
    // cde encoded tag and 32-byte key data
    let encoded = b"keeA48J-DbADmUdvXrUjJ_r4XkZv4TEtHRVoFS6oQ0AgY5i";

    // decode the tag first...
    let tag = TagBuilder::from_encoded(encoded).build().unwrap();

    // local buffer used for decoding the tag and data into
    let mut buf = [0u8; 32];
    assert_eq!(buf.len(), tag.get_data_length() as usize);

    // decode the rest of the string
    ENCODER.decode_mut(&encoded[tag.encode_len()..], &mut buf).unwrap();

    // create the key from the bytes
    let key = Key::from(&buf[..]);

    assert_eq!("key.ed25519.secret", format!("{}", tag));
    assert_eq!("7bda5f8c18233340d5dd1d09a7f45edcae557b39139f1d4e972ecec1a806e3a2", format!("{}", key));
}

