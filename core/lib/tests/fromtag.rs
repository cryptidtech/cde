mod fromtag {
    use cde::{idx, TagBuilder};

    #[test]
    #[should_panic]
    fn empty_string() {
        let _tt = TagBuilder::from_tag("").build().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_subclass() {
        let _tt = TagBuilder::from_tag("key").build().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_subsubclass() {
        let _tt = TagBuilder::from_tag("key.ed25519").build().unwrap();
    }

    #[test]
    #[should_panic]
    fn nonexp_nonstd_class_nonexp_nonstd_subclass() {
        let _tt = TagBuilder::from_tag("foo.bar").build().unwrap();
    }

    #[test]
    #[should_panic]
    fn exp_nonstd_class_nonexp_nonstd_subclass() {
        let _tt = TagBuilder::from_tag("Foo.bar").build().unwrap();
    }

    #[test]
    fn exp_nonstd_class_exp_nonstd_subclass() {
        let tt = TagBuilder::from_tag("Foo.Bar").build().unwrap();
        assert_eq!(idx('F'), tt.class());
        assert_eq!(idx('B'), tt.subclass());
        assert_eq!(0, tt.subsubclass());
    }

    #[test]
    #[should_panic]
    fn nonexp_std_class_nonexp_nonstd_subclass() {
        let _tt = TagBuilder::from_tag("key.foo").build().unwrap();
    }

    #[test]
    #[should_panic]
    fn exp_std_class_nonexp_nonstd_subclass() {
        let _tt = TagBuilder::from_tag("Key.foo").build().unwrap();
    }

    #[test]
    fn exp_std_class_exp_nonstd_subclass() {
        let tt = TagBuilder::from_tag("Key.Bar").build().unwrap();
        assert_eq!(idx('K'), tt.class());
        assert_eq!(idx('B'), tt.subclass());
        assert_eq!(0, tt.subsubclass());
    }

    #[test]
    #[should_panic]
    fn nonexp_std_class_nonexp_std_subclass_no_subsubclasses_nonstd_subsubclass() {
        let _tt = TagBuilder::from_tag("claim.oberon.0").build().unwrap();
    }

    #[test]
    fn nonexp_std_class_exp_std_subclass_no_subclasses_nonstd_subsubclass() {
        let tt = TagBuilder::from_tag("claim.Oberon.0").build().unwrap();
        assert_eq!(idx('c'), tt.class());
        assert_eq!(idx('O'), tt.subclass());
        assert_eq!(0, tt.subsubclass());
    }

    #[test]
    fn nonexp_std_class_nonexp_std_subclass_no_subclasses() {
        let tt = TagBuilder::from_tag("claim.oberon").build().unwrap();
        assert_eq!(idx('c'), tt.class());
        assert_eq!(idx('o'), tt.subclass());
        assert_eq!(0, tt.subsubclass());
    }

    #[test]
    fn undefined_undefined() {
        let tt = TagBuilder::from_tag("undefined.undefined").build().unwrap();
        assert_eq!(idx('_'), tt.class());
        assert_eq!(idx('_'), tt.subclass());
        assert_eq!(0, tt.subsubclass());
    }

    #[test]
    fn undefined_list() {
        let tt = TagBuilder::from_tag("undefined.list").build().unwrap();
        assert_eq!(idx('_'), tt.class());
        assert_eq!(idx('-'), tt.subclass());
        assert_eq!(0, tt.subsubclass());
    }

    #[test]
    fn list_list() {
        let tt = TagBuilder::from_tag("list.list").build().unwrap();
        assert_eq!(idx('-'), tt.class());
        assert_eq!(idx('-'), tt.subclass());
        assert_eq!(0, tt.subsubclass());
    }

    #[test]
    #[should_panic]
    fn list_undefined() {
        let _tt = TagBuilder::from_tag("list.undefined").build().unwrap();
    }

    #[test]
    fn undefined_undefined_nonstd_subsubclass() {
        let tt = TagBuilder::from_tag("undefined.undefined.3")
            .build()
            .unwrap();
        assert_eq!(idx('_'), tt.class());
        assert_eq!(idx('_'), tt.subclass());
        assert_eq!(3, tt.subsubclass());
    }

    #[test]
    fn undefined_list_nonstd_subsubclass() {
        let tt = TagBuilder::from_tag("undefined.list.0").build().unwrap();
        assert_eq!(idx('_'), tt.class());
        assert_eq!(idx('-'), tt.subclass());
        assert_eq!(0, tt.subsubclass());
    }

    #[test]
    fn list_list_nonstd_subsubclass() {
        let tt = TagBuilder::from_tag("list.list.2").build().unwrap();
        assert_eq!(idx('-'), tt.class());
        assert_eq!(idx('-'), tt.subclass());
        assert_eq!(2, tt.subsubclass());
    }

    #[test]
    fn nonexp_std_class_nonexp_std_subclass_std_subsubclass() {
        let tt = TagBuilder::from_tag("key.ed25519.public").build().unwrap();
        assert_eq!(idx('k'), tt.class());
        assert_eq!(idx('e'), tt.subclass());
        assert_eq!(0, tt.subsubclass());
    }

    #[test]
    fn nonexp_std_class_exp_std_subclass_nonstd_subsubclass() {
        let tt = TagBuilder::from_tag("key.Ed25519.2").build().unwrap();
        assert_eq!(idx('k'), tt.class());
        assert_eq!(idx('E'), tt.subclass());
        assert_eq!(2, tt.subsubclass());
    }

    #[test]
    #[should_panic]
    fn exp_std_class_and_nonexp_std_subclass() {
        let _tt = TagBuilder::from_tag("Key.ed25519").build().unwrap();
    }

    #[test]
    #[should_panic]
    fn nonexp_nonstd_class_nonexp_nonstd_subclass_nonstd_subsubclass() {
        let _tt = TagBuilder::from_tag("0.0.0").build().unwrap();
    }

    #[test]
    fn exp_nonstd_class_exp_nonstd_subclass_nonstd_subsubclass() {
        let tt = TagBuilder::from_tag("5.5.5").build().unwrap();
        assert_eq!(idx('5'), tt.class());
        assert_eq!(idx('5'), tt.subclass());
        assert_eq!(5, tt.subsubclass());
    }

    #[test]
    fn exp_std_class_exp_nonstd_subclass_nonstd_subsubclass() {
        let tt = TagBuilder::from_tag("Key.Foo.3").build().unwrap();
        assert_eq!(idx('K'), tt.class());
        assert_eq!(idx('F'), tt.subclass());
        assert_eq!(3, tt.subsubclass());
    }

    #[test]
    fn print_class_subclass_subsubclass() {
        let tt = TagBuilder::from_tag("key.ed25519.secret").build().unwrap();
        assert_eq!("key.ed25519.secret", format!("{}", tt));
    }

    #[test]
    fn print_exp_class_subclass_subsubclass() {
        let tt = TagBuilder::from_tag("Key.Foo").build().unwrap();
        assert_eq!("Key.F", format!("{}", tt));
    }

    #[test]
    fn print_list_list_exp_subsubclass() {
        let tt = TagBuilder::from_tag("list.list.5").build().unwrap();
        assert_eq!("list.list.5", format!("{}", tt));
    }
}
