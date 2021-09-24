use cde::{ idx, Tag };
use std::str::FromStr;

#[test]
#[should_panic]
fn just_class() {
    let _tt = Tag::from_str("key").unwrap();
}

#[test]
fn empty() {
    let tt = Tag::from_str("").unwrap();
    assert_eq!(idx('_'), tt.class());
    assert_eq!(idx('_'), tt.subclass());
    assert_eq!(0, tt.subsubclass());
}

#[test]
fn numerical_class() {
    let tt = Tag::from_str("10.ed25519").unwrap();
    assert_eq!(idx('k'), tt.class());
    assert_eq!(idx('e'), tt.subclass());
    assert_eq!(0, tt.subsubclass());
}

#[test]
fn exp_numerical_class() {
    let tt = Tag::from_str("42.ed25519").unwrap();
    assert_eq!(idx('K'), tt.class());
    assert_eq!(idx('e'), tt.subclass());
    assert_eq!(0, tt.subsubclass());
}

#[test]
fn char_class() {
    let tt = Tag::from_str("k.ed25519").unwrap();
    assert_eq!(idx('k'), tt.class());
    assert_eq!(idx('e'), tt.subclass());
    assert_eq!(0, tt.subsubclass());
}

#[test]
fn exp_char_class() {
    let tt = Tag::from_str("K.foo").unwrap();
    assert_eq!(idx('K'), tt.class());
    assert_eq!(idx('f'), tt.subclass());
    assert_eq!(0, tt.subsubclass());
}

#[test]
#[should_panic]
fn non_exp_char_class() {
    let _tt = Tag::from_str("k.foo").unwrap();
}

#[test]
fn class_and_subclass() {
    let tt = Tag::from_str("key.ed25519").unwrap();
    assert_eq!(idx('k'), tt.class());
    assert_eq!(idx('e'), tt.subclass());
    assert_eq!(0, tt.subsubclass());
}

#[test]
fn class_subclass_subsubclass() {
    let tt = Tag::from_str("key.ed25519.public").unwrap();
    assert_eq!(idx('k'), tt.class());
    assert_eq!(idx('e'), tt.subclass());
    assert_eq!(0, tt.subsubclass());
}

#[test]
fn class_and_exp_subclass() {
    let tt = Tag::from_str("key.ed25519.2").unwrap();
    assert_eq!(idx('k'), tt.class());
    assert_eq!(idx('e'), tt.subclass());
    assert_eq!(2, tt.subsubclass());
}

#[test]
fn exp_class_and_non_exp_subclass() {
    let tt = Tag::from_str("Key.ed25519.2").unwrap();
    assert_eq!(idx('K'), tt.class());
    assert_eq!(idx('e'), tt.subclass());
    assert_eq!(2, tt.subsubclass());
}

#[test]
fn exp_class() {
    let tt = Tag::from_str("Key.foo").unwrap();
    assert_eq!(idx('K'), tt.class());
    assert_eq!(idx('f'), tt.subclass());
    assert_eq!(0, tt.subsubclass());
}

#[test]
fn exp_char_class_subclass() {
    let tt = Tag::from_str("K.F.3").unwrap();
    assert_eq!(idx('K'), tt.class());
    assert_eq!(idx('F'), tt.subclass());
    assert_eq!(3, tt.subsubclass());
}

#[test]
fn numeric_class_subclass_subsubclass() {
    let tt = Tag::from_str("0.0.0").unwrap();
    assert_eq!(idx('a'), tt.class());
    assert_eq!(idx('a'), tt.subclass());
    assert_eq!(0, tt.subsubclass());
}

#[test]
#[should_panic]
fn non_exp_class() {
    let _tt = Tag::from_str("key.foo").unwrap();
}

#[test]
fn exp_class_subclass() {
    let tt = Tag::from_str("Key.Foo.3").unwrap();
    assert_eq!(idx('K'), tt.class());
    assert_eq!(idx('F'), tt.subclass());
    assert_eq!(3, tt.subsubclass());
}

#[test]
#[should_panic]
fn list_any() {
    let _tt = Tag::from_str("list.any").unwrap();
}

#[test]
fn list_list() {
    let tt = Tag::from_str("-.-").unwrap();
    assert_eq!(idx('-'), tt.class());
    assert_eq!(idx('-'), tt.subclass());
    assert_eq!(0, tt.subsubclass());
}

#[test]
fn list_list_subsubclass() {
    let tt = Tag::from_str("-.-.0").unwrap();
    assert_eq!(idx('-'), tt.class());
    assert_eq!(idx('-'), tt.subclass());
    assert_eq!(0, tt.subsubclass());
}

#[test]
fn list_list_exp_subsubclass() {
    let tt = Tag::from_str("-.-.5").unwrap();
    assert_eq!(idx('-'), tt.class());
    assert_eq!(idx('-'), tt.subclass());
    assert_eq!(5, tt.subsubclass());
}

#[test]
fn print_class_subclass_subsubclass() {
    let tt = Tag::from_str("key.ed25519").unwrap();
    assert_eq!("key.ed25519.0", format!("{}", tt));
}

#[test]
fn print_exp_class_subclass_subsubclass() {
    let tt = Tag::from_str("Key.Foo").unwrap();
    assert_eq!("Key.Foo.0", format!("{}", tt));
}

#[test]
fn print_empty() {
    let tt = Tag::from_str("").unwrap();
    assert_eq!("_._.0", format!("{}", tt));
}

#[test]
fn print_any() {
    let tt = Tag::from_str("_").unwrap();
    assert_eq!("_._.0", format!("{}", tt));
}

#[test]
fn print_list_list_exp_subsubclass() {
    let tt = Tag::from_str("-.-.5").unwrap();
    assert_eq!("-.-.5", format!("{}", tt));
}


