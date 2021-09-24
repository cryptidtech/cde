#[macro_use] extern crate cde_codegen;
use cde::{ encoder, TagBuilder, CryptoData };
use rand::{ thread_rng, Rng };

#[cde("key.ed25519.public")]
struct Key {
    data: [u8; 32]
}

impl Key {
    fn new() -> Self {
        let mut arr = [0u8; 32];
        thread_rng().try_fill(&mut arr[..]).unwrap();
        Key {
            data: arr
        }
    }

    fn len(&self) -> usize {
        32
    }

    fn encode(&self) -> String {
        let e = encoder().unwrap();
        e.encode(&self.data)
    }
}

fn main() {
    // generate a random key
    let k = Key::new();

    // create a tag for it
    let t = TagBuilder::from_str(&k.tag()).length(k.len() as u32).build();

    println!("\n{:?}\n", t);

    // encode the tag followed by the key data
    print!("{}{}", t.encode(), k.encode());
}
