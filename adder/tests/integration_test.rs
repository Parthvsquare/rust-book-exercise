extern crate adder;

#[test]
fn it_adds_two_external() {
    assert_eq!(6, adder::add(4, 2))
}
