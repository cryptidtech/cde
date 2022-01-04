use cde::{ENCODER, CryptoData, Tag, TagBuilder};
use rand::{thread_rng, Rng};

struct Key {
    tag: Tag,
    data: [u8; 32],
}

impl CryptoData for Key {
    fn len(&self) -> usize {
        32
    }

    fn encode_len(&self) -> usize {
        self.tag.encode_len() + ENCODER.encode_len(self.len())
    }

    fn encode(&self, out: &mut [u8]) {
        if self.tag.is_extended() {
            self.tag.encode(&mut out[..8]);
            ENCODER.encode_mut(&self.data, &mut out[8..8+ENCODER.encode_len(self.len())]);
        } else {
            self.tag.encode(&mut out[..4]);
            ENCODER.encode_mut(&self.data, &mut out[4..4+ENCODER.encode_len(self.len())]);
        }
    }
}

impl Key {
    fn new() -> Self {
        let mut arr = [0u8; 32];
        thread_rng().try_fill(&mut arr[..]).unwrap();
        Key {
            tag: TagBuilder::from_str("key.ed25519.secret").length(32).build().unwrap(),
            data: arr
        }
    }
}

fn main() {
    // generate a random key
    let k = Key::new();
    let mut b = [0u8; 128];
    k.encode(&mut b[..k.encode_len()]);
    let s = core::str::from_utf8(&b[..k.encode_len()]).unwrap();
    println!("{}", s);
}
