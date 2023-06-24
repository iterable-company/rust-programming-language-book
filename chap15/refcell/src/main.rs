pub mod messenger;

fn main() {
    check_mutable_and_immutable_boroow_of_box();
    check_mutable_and_immutable_boroow_of_box_by_using_function();
    combine_rc_and_refcell();
    construct_list_using_refcell_only();
}

fn check_mutable_and_immutable_boroow_of_box() {
    // can't compile because 'println!(x)' is borrowing as immutable after mutable borrow ('let y')
    {
        let mut a = Box::new("hoge".to_string());
        let x = &a;
        // let y = &mut a;  この行を有効にすると、可変借用の後に不変借用が行われることになりコンパイルエラー
        println!("{}", x);
    }
    // can compile because immutable borrow isn't occurred after mutable borrow ('let y')
    {
        let mut a = Box::new("hoge".to_string());
        let _x = &a;
        let y = &mut a;
        // println!("{}", x);  この行を有効にすると、可変借用の後に不変借用が行われることになりコンパイルエラー
        println!("{}", y);
    }
}

fn check_mutable_and_immutable_boroow_of_box_by_using_function() {
    let mut a = Box::new(1);
    borrow_immutable(&a);
    borrow_mutable(&mut a);
    borrow_immutable(&a);
    mutate_value(&mut a);
    borrow_immutable(&a);
    println!("{:?}", a);
}

fn borrow_mutable(a: &mut Box<i32>) {
    *a = Box::new(2);
    println!("{:?}", a);
}

fn mutate_value(a: &mut Box<i32>) {
    **a = **a + 1;
    println!("{:?}", a);
}

fn borrow_immutable(a: &Box<i32>) {
    println!("{:?}", a);
}

use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug)]
enum RefCellList {
    Cons(Rc<RefCell<i32>>, Rc<RefCellList>),
    Nil,
}

fn combine_rc_and_refcell() {
    use RefCellList::{Cons, Nil};

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

#[derive(Debug)]
enum List {
    Cons(Rc<i32>, Rc<List>),
    Nil,
}

fn construct_list_using_refcell_only() {
    use List::{Cons, Nil};

    let value = Rc::new(5);

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(6), Rc::clone(&a));
    let c = Cons(Rc::new(10), Rc::clone(&a));

    // *value += 10; Rcには値をassignできない。DerefMut トレイトが実装されていないといけない

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
