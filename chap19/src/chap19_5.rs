use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

pub fn run() {
    Pancakes::hello_macro();
}
