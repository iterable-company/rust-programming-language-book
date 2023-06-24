extern crate adder_test;

#[test]
fn it_adds_two_2() {
    assert_eq!(4, adder_test::adder::add_two(2))
}