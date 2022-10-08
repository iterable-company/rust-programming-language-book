#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::todo;
fn main() {
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["", "\n"],
            &[::core::fmt::ArgumentV1::new_display(&get_string())],
        ));
    };
}
fn get_string() -> String {
    ::core::panicking::panic("not yet implemented")
}
