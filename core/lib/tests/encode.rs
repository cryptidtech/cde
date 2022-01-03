use cde::Tag;
use std::str::FromStr;

#[test]
fn encode1() {
    let mut tt = Tag::from_str("Foo.Bar").unwrap();
    tt.set_length(1024u32);

    let mut b = [0u8; 8];
    tt.encode(&mut b);
    let s = core::str::from_utf8(&b).unwrap();

    assert_eq!("FBAaaaqa", s);
}

#[test]
fn encode2() {
    let mut tt = Tag::from_str("key.ed25519.secret").unwrap();
    tt.set_length(32);

    let mut b = [0u8; 4];
    tt.encode(&mut b);
    let s = core::str::from_utf8(&b).unwrap();

    assert_eq!("keeA", s);
}

#[test]
fn encode3() {
    let mut tt = Tag::from_str("key.ed25519.public").unwrap();
    tt.set_length(32);

    let mut b = [0u8; 4];
    tt.encode(&mut b);
    let s = core::str::from_utf8(&b).unwrap();

    assert_eq!("keaA", s);
}

#[test]
fn encode4() {
    let mut tt = Tag::from_str("claim.oberon").unwrap();
    // Oberon proofs are 380 bytes long
    tt.set_length(380);

    let mut b = [0u8; 8];
    tt.encode(&mut b);
    let s = core::str::from_utf8(&b).unwrap();

    assert_eq!("coAaaaf7", s);
}

#[test]
fn encode5() {
    let mut tt = Tag::from_str("claim.Oberon.1").unwrap();
    // experimental double Oberon proof that is 760 bytes long
    tt.set_length(760);

    let mut b = [0u8; 8];
    tt.encode(&mut b);
    let s = core::str::from_utf8(&b).unwrap();

    assert_eq!("cOEaaalY", s);
}

#[test]
fn encode6() {
    let mut tt = Tag::from_str("list.list.2").unwrap();
    // a list with five lists with sub-sub-class of 2
    tt.set_length(5);

    let mut b = [0u8; 4];
    tt.encode(&mut b);
    let s = core::str::from_utf8(&b).unwrap();

    assert_eq!("--if", s);
}

#[test]
fn encode7() {
    let mut tt = Tag::from_str("list.list").unwrap();
    // a list with 200 lists
    tt.set_length(200);

    let mut b = [0u8; 4];
    tt.encode(&mut b);
    let s = core::str::from_utf8(&b).unwrap();

    assert_eq!("--di", s);
}

#[test]
fn encode8() {
    let mut tt = Tag::from_str("undefined.undefined").unwrap();
    // a BLOB that is 1GB in size
    tt.set_length(1024*1024*1024);

    let mut b = [0u8; 8];
    tt.encode(&mut b);
    let s = core::str::from_utf8(&b).unwrap();

    assert_eq!("__Baaaaa", s);
}



