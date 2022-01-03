#[macro_use] extern crate cde_codegen;
use cde::{ ENCODER, CryptoData, CryptoDataTag, Tag };
use rand::{ thread_rng, Rng };

//#[cde("key.ed25519.public")]
struct Key {
    data: [u8; 32]
}

let mut key_type_tag = Tag::from_str("key.ed25519.public").unwrap();

impl<T: CryptoData> CryptoDataTag for Key {
    fn as_str(&self) -> &str {
        key_type_tag.set_length((self as T).get_length());
        key_type_tag.as_str()
    }

    fn as_bytes(&self) -> &[u8] {
        key_type_tag.set_length((self as T).get_length());
        key_type_tag.as_bytes()
    }
}

impl CryptoData for Key {
    fn get_length(&self) -> usize {
        32
    }

    fn encode(&self, out: &mut [u8]) {
        ENCODER.encode(&self.data, out);
    }
}

impl Key {
    fn new() -> Self {
        let mut arr = [0u8; 32];
        thread_rng().try_fill(&mut arr[..]).unwrap();
        Key {
            data: arr
        }
    }

}

fn main() {
    // generate a random key
    let _k = Key::new();

    let mut _b = [0u8; 47];


    // encode the tag followed by the key data
    //print!("{}{}", t.encode(), k.encode());
}
