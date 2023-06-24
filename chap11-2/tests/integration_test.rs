extern crate adder_test;

mod common;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder_test::adder::add_two(common::get_two()))
}