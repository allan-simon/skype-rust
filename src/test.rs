use rc4::Rc4;

#[test]
fn rc4() {
    let mut rc4 = Rc4::new("key".as_ref());
    let crypted = rc4.crypt("text".as_ref());
    assert_eq!(crypted, [127, 9, 76, 153]);
}
