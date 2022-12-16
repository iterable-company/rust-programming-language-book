use std::ops::Deref;

fn main() {
    trace_pointer();
    use_box();
    mybox();
    mybox1();
    implicit_deref_coercion();
}

fn trace_pointer() {
    let x = 5;
    let y = &x;

    println!("{}", x);
    println!("{}", y);
    println!("{}", *y);

    assert_eq!(5, x);
    //assert_eq!(5, y); // compile error 数値と数値への参照は異なる型のため、比較できない
    assert_eq!(5, *y);
}

fn use_box() {
    let x = 5;
    let y = Box::new(x);

    println!("{}", x);
    println!("{}", y);
    println!("{}", *y);

    assert_eq!(5, x);
    //assert_eq!(5, y); // compile error Box<i32> と i32 は型が異なる
    assert_eq!(5, *y);
    assert_eq!(Box::new(5), y); // Box<i32> でも参照外し演算を使うと i32 として比較できる
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn mybox() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // assert_eq!(5, *y); // compile error! MyBox型の参照外しの方法を知らない
}

#[derive(Debug, PartialEq)]
struct MyBox1<T>(T);
impl<T> MyBox1<T> {
    fn new(x: T) -> MyBox1<T> {
        MyBox1(x)
    }
}
impl<T> Deref for MyBox1<T> {
    type Target = T;

    // 参照を返さないと包まれている値の所有権をmoveしてしまうことになる
    fn deref(&self) -> &T {
        &self.0
    }
} // *y の時、 *(y.deref()) を自動で呼び出してくれる

fn mybox1() {
    let x = 5;
    let y = MyBox1::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // deref() で参照が返せるため、OK
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn implicit_deref_coercion() {
    let m = MyBox1::new(String::from("hoge"));
    hello(&m) // &m => &(MyBox1::deref()) -> &String -> &(String::deref()) -> &str 強制的に変換してくれる。しかも &&にならない
}
