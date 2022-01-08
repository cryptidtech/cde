use cde::{CryptoData, ENCODER, encode_tag_and_data, TagBuilder};

#[test]
fn encode1() {
    let mut tt = TagBuilder::from_tag("Foo.Bar").build().unwrap();
    tt.set_data_length(1024);

    let mut b = [0u8; 8];
    let len = tt.encode(&mut b);
    assert_eq!(len, tt.encode_len());
    let s = core::str::from_utf8(&b).unwrap();

    assert_eq!("FBcacaaa", s);
}

#[test]
fn encode2() {
    let mut tt = TagBuilder::from_tag("key.ed25519.secret").build().unwrap();
    tt.set_data_length(32);

    let mut b = [0u8; 4];
    let len = tt.encode(&mut b);
    assert_eq!(len, tt.encode_len());
    let s = core::str::from_utf8(&b).unwrap();

    assert_eq!("keeA", s);
}

#[test]
fn encode3() {
    let mut tt = TagBuilder::from_tag("key.ed25519.public").build().unwrap();
    tt.set_data_length(32);

    let mut b = [0u8; 4];
    let len = tt.encode(&mut b);
    assert_eq!(len, tt.encode_len());
    let s = core::str::from_utf8(&b).unwrap();

    assert_eq!("keaA", s);
}

#[test]
fn encode4() {
    let mut tt = TagBuilder::from_tag("claim.oberon").build().unwrap();
    // Oberon proofs are 380 bytes long
    tt.set_data_length(380);

    let mut b = [0u8; 8];
    let len = tt.encode(&mut b);
    assert_eq!(len, tt.encode_len());
    let s = core::str::from_utf8(&b).unwrap();

    assert_eq!("cod7aAaa", s);
}

#[test]
fn encode5() {
    let mut tt = TagBuilder::from_tag("claim.Oberon.1").build().unwrap();
    // experimental double Oberon proof that is 760 bytes long
    tt.set_data_length(760);

    let mut b = [0u8; 8];
    let len = tt.encode(&mut b);
    assert_eq!(len, tt.encode_len());
    let s = core::str::from_utf8(&b).unwrap();

    assert_eq!("cOhYbqaa", s);
}

#[test]
fn encode6() {
    let mut tt = TagBuilder::from_tag("list.list.2").build().unwrap();
    // a list with five lists with sub-sub-class of 2
    tt.set_data_length(5);

    let mut b = [0u8; 4];
    let len = tt.encode(&mut b);
    assert_eq!(len, tt.encode_len());
    let s = core::str::from_utf8(&b).unwrap();

    assert_eq!("--if", s);
}

#[test]
fn encode7() {
    let mut tt = TagBuilder::from_tag("list.list").build().unwrap();
    // a list with 200 lists
    tt.set_data_length(200);

    let mut b = [0u8; 8];
    let len = tt.encode(&mut b);
    assert_eq!(len, tt.encode_len());
    let s = core::str::from_utf8(&b).unwrap();

    assert_eq!("--diaqaa", s);
}

#[test]
fn encode8() {
    let mut tt = TagBuilder::from_tag("undefined.undefined").build().unwrap();
    // a BLOB that is 1GB in size
    tt.set_data_length(1024*1024*1024);

    let mut b = [0u8; 12];
    let len = tt.encode(&mut b);
    assert_eq!(len, tt.encode_len());
    let s = core::str::from_utf8(&b).unwrap();

    assert_eq!("__caAicabaaa", s);
}

#[test]
fn encode9() {

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
    impl CryptoData for Key {
        fn len(&self) -> usize { 32 }
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

    let mut key = Key::default();
    ENCODER.decode_mut(b"48J-DbADmUdvXrUjJ_r4XkZv4TEtHRVoFS6oQ0AgY5i", key.as_mut()).unwrap();

    let mut tag = TagBuilder::from_tag("key.ed25519.secret").build().unwrap();

    let mut b = [0u8; 47];
    let len = encode_tag_and_data(&mut tag, &key, &mut b).unwrap();
    assert_eq!(len, b.len());

    let s = core::str::from_utf8(&b).unwrap();

    assert_eq!("keeA48J-DbADmUdvXrUjJ_r4XkZv4TEtHRVoFS6oQ0AgY5i", s);
}
