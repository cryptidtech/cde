mod frombytes {
    use cde::TagBuilder;

    #[test]
    #[should_panic]
    fn not_enough_bytes() {
        let b = [0x0a, 0x11];
        let _tt = TagBuilder::from_bytes(&b).build().unwrap();
    }

    #[test]
    #[should_panic]
    fn incomplete_one_byte_length() {
        let b = [0x0a, 0x11, 0x80];
        let _tt = TagBuilder::from_bytes(&b).build().unwrap();
    }

    #[test]
    fn one_byte_length() {
        let b = [0x0a, 0x11, 0x01];
        let _tt = TagBuilder::from_bytes(&b).build().unwrap();
    }

    #[test]
    #[should_panic]
    fn incomplete_two_byte_length() {
        let b = [0x0a, 0x11, 0x80, 0x80];
        let _tt = TagBuilder::from_bytes(&b).build().unwrap();
    }

    #[test]
    fn two_byte_length() {
        let b = [0x0a, 0x11, 0x80, 0x01];
        let _tt = TagBuilder::from_bytes(&b).build().unwrap();
    }

    #[test]
    #[should_panic]
    fn incomplete_three_byte_length() {
        let b = [0x0a, 0x11, 0x80, 0x80, 0x80];
        let _tt = TagBuilder::from_bytes(&b).build().unwrap();
    }

    #[test]
    fn three_byte_length() {
        let b = [0x0a, 0x11, 0x80, 0x80, 0x01];
        let _tt = TagBuilder::from_bytes(&b).build().unwrap();
    }

    #[test]
    #[should_panic]
    fn incomplete_four_byte_length() {
        let b = [0x0a, 0x11, 0x80, 0x80, 0x80, 0x80];
        let _tt = TagBuilder::from_bytes(&b).build().unwrap();
    }

    #[test]
    fn four_byte_length() {
        let b = [0x0a, 0x11, 0x80, 0x80, 0x80, 0x01];
        let _tt = TagBuilder::from_bytes(&b).build().unwrap();
    }

    #[test]
    #[should_panic]
    fn incomplete_five_byte_length() {
        let b = [0x0a, 0x11, 0x80, 0x80, 0x80, 0x80, 0x80];
        let _tt = TagBuilder::from_bytes(&b).build().unwrap();
    }

    #[test]
    fn five_byte_length() {
        let b = [0x0a, 0x11, 0x80, 0x80, 0x80, 0x80, 0x01];
        let _tt = TagBuilder::from_bytes(&b).build().unwrap();
    }

    #[test]
    #[should_panic]
    fn incomplete_six_byte_length() {
        let b = [0x0a, 0x11, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80];
        let _tt = TagBuilder::from_bytes(&b).build().unwrap();
    }

    #[test]
    fn six_byte_length() {
        let b = [0x0a, 0x11, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01];
        let _tt = TagBuilder::from_bytes(&b).build().unwrap();
    }

    #[test]
    #[should_panic]
    fn incomplete_seven_byte_length() {
        let b = [0x0a, 0x11, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80];
        let _tt = TagBuilder::from_bytes(&b).build().unwrap();
    }

    #[test]
    fn seven_byte_length() {
        let b = [0x0a, 0x11, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01];
        let _tt = TagBuilder::from_bytes(&b).build().unwrap();
    }
}
