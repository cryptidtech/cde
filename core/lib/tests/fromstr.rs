use cde::{ idx, Tag };
use std::str::FromStr;

#[test]
#[should_panic]
fn empty_string() {
    let _tt = Tag::from_str("").unwrap();
}

#[test]
#[should_panic]
fn missing_subclass() {
    let _tt = Tag::from_str("key").unwrap();
}

#[test]
#[should_panic]
fn missing_subsubclass() {
    let _tt = Tag::from_str("key.ed25519").unwrap();
}

#[test]
#[should_panic]
fn nonexp_nonstd_class_nonexp_nonstd_subclass() {
    let _tt = Tag::from_str("foo.bar").unwrap();
}

#[test]
#[should_panic]
fn exp_nonstd_class_nonexp_nonstd_subclass() {
    let _tt = Tag::from_str("Foo.bar").unwrap();
}

#[test]
fn exp_nonstd_class_exp_nonstd_subclass() {
    let tt = Tag::from_str("Foo.Bar").unwrap();
    assert_eq!(idx('F'), tt.class());
    assert_eq!(idx('B'), tt.subclass());
    assert_eq!(0, tt.subsubclass());
}

#[test]
#[should_panic]
fn nonexp_std_class_nonexp_nonstd_subclass() {
    let _tt = Tag::from_str("key.foo").unwrap();
}

#[test]
#[should_panic]
fn exp_std_class_nonexp_nonstd_subclass() {
    let _tt = Tag::from_str("Key.foo").unwrap();
}

#[test]
fn exp_std_class_exp_nonstd_subclass() {
    let tt = Tag::from_str("Key.Bar").unwrap();
    assert_eq!(idx('K'), tt.class());
    assert_eq!(idx('B'), tt.subclass());
    assert_eq!(0, tt.subsubclass());
}

#[test]
#[should_panic]
fn nonexp_std_class_nonexp_std_subclass_no_subsubclasses_nonstd_subsubclass() {
    let _tt = Tag::from_str("claim.oberon.0").unwrap();
}

#[test]
fn nonexp_std_class_exp_std_subclass_no_subclasses_nonstd_subsubclass() {
    let tt = Tag::from_str("claim.Oberon.0").unwrap();
    assert_eq!(idx('c'), tt.class());
    assert_eq!(idx('O'), tt.subclass());
    assert_eq!(0, tt.subsubclass());
}

#[test]
fn nonexp_std_class_nonexp_std_subclass_no_subclasses() {
    let tt = Tag::from_str("claim.oberon").unwrap();
    assert_eq!(idx('c'), tt.class());
    assert_eq!(idx('o'), tt.subclass());
    assert_eq!(0, tt.subsubclass());
}

#[test]
fn undefined_undefined() {
    let tt = Tag::from_str("undefined.undefined").unwrap();
    assert_eq!(idx('_'), tt.class());
    assert_eq!(idx('_'), tt.subclass());
    assert_eq!(0, tt.subsubclass());
}

#[test]
fn undefined_list() {
    let tt = Tag::from_str("undefined.list").unwrap();
    assert_eq!(idx('_'), tt.class());
    assert_eq!(idx('-'), tt.subclass());
    assert_eq!(0, tt.subsubclass());
}

#[test]
fn list_list() {
    let tt = Tag::from_str("list.list").unwrap();
    assert_eq!(idx('-'), tt.class());
    assert_eq!(idx('-'), tt.subclass());
    assert_eq!(0, tt.subsubclass());
}

#[test]
#[should_panic]
fn list_undefined() {
    let _tt = Tag::from_str("list.undefined").unwrap();
}

#[test]
fn undefined_undefined_nonstd_subsubclass() {
    let tt = Tag::from_str("undefined.undefined.3").unwrap();
    assert_eq!(idx('_'), tt.class());
    assert_eq!(idx('_'), tt.subclass());
    assert_eq!(3, tt.subsubclass());
}

#[test]
fn undefined_list_nonstd_subsubclass() {
    let tt = Tag::from_str("undefined.list.0").unwrap();
    assert_eq!(idx('_'), tt.class());
    assert_eq!(idx('-'), tt.subclass());
    assert_eq!(0, tt.subsubclass());
}

#[test]
fn list_list_nonstd_subsubclass() {
    let tt = Tag::from_str("list.list.2").unwrap();
    assert_eq!(idx('-'), tt.class());
    assert_eq!(idx('-'), tt.subclass());
    assert_eq!(2, tt.subsubclass());
}

#[test]
fn nonexp_std_class_nonexp_std_subclass_std_subsubclass() {
    let tt = Tag::from_str("key.ed25519.public").unwrap();
    assert_eq!(idx('k'), tt.class());
    assert_eq!(idx('e'), tt.subclass());
    assert_eq!(0, tt.subsubclass());
}

#[test]
fn nonexp_std_class_exp_std_subclass_nonstd_subsubclass() {
    let tt = Tag::from_str("key.Ed25519.2").unwrap();
    assert_eq!(idx('k'), tt.class());
    assert_eq!(idx('E'), tt.subclass());
    assert_eq!(2, tt.subsubclass());
}

#[test]
#[should_panic]
fn exp_std_class_and_nonexp_std_subclass() {
    let _tt = Tag::from_str("Key.ed25519").unwrap();
}

#[test]
#[should_panic]
fn nonexp_nonstd_class_nonexp_nonstd_subclass_nonstd_subsubclass() {
    let _tt = Tag::from_str("0.0.0").unwrap();
}

#[test]
fn exp_nonstd_class_exp_nonstd_subclass_nonstd_subsubclass() {
    let tt = Tag::from_str("5.5.5").unwrap();
    assert_eq!(idx('5'), tt.class());
    assert_eq!(idx('5'), tt.subclass());
    assert_eq!(5, tt.subsubclass());
}

#[test]
fn exp_std_class_exp_nonstd_subclass_nonstd_subsubclass() {
    let tt = Tag::from_str("Key.Foo.3").unwrap();
    assert_eq!(idx('K'), tt.class());
    assert_eq!(idx('F'), tt.subclass());
    assert_eq!(3, tt.subsubclass());
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
fn print_list_list_exp_subsubclass() {
    let tt = Tag::from_str("list.list.5").unwrap();
    assert_eq!("list.list.5", format!("{}", tt));
}
