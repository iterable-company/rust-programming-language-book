fn main() {
    compile_error_by_using_box_new_to_moved_instance();
    enable_to_compile_by_using_rc();
    show_referencd_counter();
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn compile_error_by_using_box_new_to_moved_instance() {
    use List::{Cons, Nil};

    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let _b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));  // compile error! 上の行で a の所有権が b にmoveしている
}

use std::rc::Rc;

#[derive(Debug)]
enum RcList {
    Cons(i32, Rc<RcList>),
    Nil,
}

fn enable_to_compile_by_using_rc() {
    use RcList::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));

    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);
}

fn show_referencd_counter() {
    use RcList::{Cons, Nil};

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
