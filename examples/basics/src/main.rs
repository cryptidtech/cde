use cde::{CryptoData, decode_tag_and_data, ENCODER, encode_tag_and_data, Tag, TagBuilder};
use core::fmt;
use core::ops::{Deref, DerefMut};
use rand::{thread_rng, Rng};

struct Key([u8; 32]);

impl Key {
    fn tag() -> Tag {
        TagBuilder::from_tag("key.x25519.secret").build().unwrap()
    }
}

impl Default for Key {
    fn default() -> Self {
        let mut key = Key([0u8; 32]);
        thread_rng().try_fill(key.as_mut()).unwrap();
        key
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

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

#[derive(Default)]
struct KeyList(Vec<Key>);

impl KeyList {
    fn tag() -> Tag {
        TagBuilder::from_tag("key.list").build().unwrap()
    }

    fn random(num: usize) -> Self {
        let mut kl = KeyList::default();
        for _i in 0..num {
            kl.0.push(Key::default());
        }
        kl
    }
}

impl Deref for KeyList {
    type Target = Vec<Key>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for KeyList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for KeyList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ ")?;
        for k in &self.0 {
            write!(f, "{}, ", k)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}

/*
impl CryptoData for KeyList {
    fn len(&self) -> usize {
        // return the number of keys in the list so that the tag has the
        // correct length value
        self.0.len()
    }
    fn bytes(&self, buf: &mut [u8]) -> usize {
        let mut idx = 0;
        for k in self.0 {
            idx += buf.copy_from_slice(k.bytes()[idx..idx+k.len()]);
        }
        idx
    }
    fn encode_len(&self) -> usize {
        ENCODER.encode_len(self.len())
    }
    fn encode(&self, buf: &mut [u8]) -> usize {
        let mut idx = 0;
        for k in self.0 {
            idx += k.encode(&mut buf[idx..idx + k.encode_len]);
        }
        self.encode_len()
    }
}
*/

fn main() {
    {
        // generate a random key
        let key = Key::default();
        let mut b = [0u8; 47];
        let mut buf = [0u8; 35];
        {
            let mut tag = Key::tag();
            println!("encode a random key:\n{}: {}", tag, key);
            encode_tag_and_data(&mut tag, &key, &mut b).unwrap();
            let s = core::str::from_utf8(&b).unwrap();
            println!("as:\n{}", s);
        }

        {
            let s = core::str::from_utf8(&b).unwrap();
            println!("decode the same key: {}", s);
            let (tag, k) = decode_tag_and_data::<Key>(&b, &mut buf).unwrap();
            println!("as:\n{}: {}", tag, k);
        }
    }

    {
        // generate a list of random keys
        let kl = KeyList::random(8);
        //let mut b = [0u8; 47];
        //let mut buf = [0u8; 35];
        {
            let tag = KeyList::tag();
            println!("encode a random key list:\n{}: {}", tag, kl);
            //tag.set_data_length( as u32);
            //let tagsize = tag.encode(buf);
        }
    }

    {
        let mut buf = [0u8; 256];
        thread_rng().try_fill(&mut buf).unwrap();
        let tag = TagBuilder::from_tag("undefined.undefined").build().unwrap();
        print!("encode random data:\n{}: ", tag);
        for b in buf {
            print!("{:02x}", b);
        }
        println!("");
    }
}
