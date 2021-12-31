use cde::{ idx, Tag };
use std::str::FromStr;

#[test]
#[should_panic]
fn just_class() {
    let _tt = Tag::from_str("key").unwrap();
}

#[test]
#[should_panic]
fn class_and_subclass_without_subsubclass() {
    let _tt = Tag::from_str("key.ed25519").unwrap();
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
    let tt = Tag::from_str("key.Ed25519.2").unwrap();
    assert_eq!(idx('k'), tt.class());
    assert_eq!(idx('E'), tt.subclass());
    assert_eq!(2, tt.subsubclass());
}

#[test]
#[should_panic]
fn exp_class_and_non_exp_subclass() {
    let _tt = Tag::from_str("Key.ed25519").unwrap();
}

#[test]
fn exp_class() {
    let tt = Tag::from_str("Key.Foo").unwrap();
    assert_eq!(idx('K'), tt.class());
    assert_eq!(idx('F'), tt.subclass());
    assert_eq!(0, tt.subsubclass());
}

#[test]
#[should_panic]
fn nonexperimental_numeric_class_subclass_subsubclass() {
    let _tt = Tag::from_str("0.0.0").unwrap();
}

#[test]
fn experimental_numeric_class_subclass_subsubclass() {
    let tt = Tag::from_str("5.5.5").unwrap();
    assert_eq!(idx('5'), tt.class());
    assert_eq!(idx('5'), tt.subclass());
    assert_eq!(5, tt.subsubclass());
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
fn list_undefined() {
    let _tt = Tag::from_str("list.undefined").unwrap();
}

#[test]
fn undefined_list() {
    let tt = Tag::from_str("undefined.list").unwrap();
    assert_eq!(idx('_'), tt.class());
    assert_eq!(idx('-'), tt.subclass());
    assert_eq!(0, tt.subsubclass());
}

#[test]
fn print_class_subclass_subsubclass() {
    let tt = Tag::from_str("key.ed25519.secret").unwrap();
    assert_eq!("key.ed25519.secret", format!("{}", tt));
}

#[test]
fn print_exp_class_subclass_subsubclass() {
    let tt = Tag::from_str("Key.Foo").unwrap();
    assert_eq!("Key.F", format!("{}", tt));
}

#[test]
#[should_panic]
fn print_empty() {
    let _tt = Tag::from_str("").unwrap();
}

#[test]
#[should_panic]
fn print_undefined() {
    let _tt = Tag::from_str("undefined").unwrap();
}

#[test]
fn print_list_list_exp_subsubclass() {
    let tt = Tag::from_str("list.list.5").unwrap();
    assert_eq!("list.list.5", format!("{}", tt));
}
