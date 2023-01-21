use std::ops::Add;

pub fn chap19_2() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
    assert_eq!(Millimeters(501) + Meters(2), Millimeters(2501));

    let person = Human;
    assert_eq!(Pilot::fly(&person), "This is your captain speaking");
    assert_eq!(Wizard::fly(&person), "Up!");
    assert_eq!(person.fly(), "*waving arms furiously*");

    full_path();
    new_type_pattern();
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// RHSとSelfが一致している実装 => 特に何も指定しないとこれになる
// trait Add<RHS=Self> {
//     type Output;
//     fn add(self, rhs: RHS) -> Self::Output;
// }

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Millimeters(u32);
struct Meters(u32);

// RHSとSelfが異なる実装
// trait Add<RHS=Self> {
//     type Output;
//     fn add(self, rhs: RHS) -> Self::Output;
// }
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// 同じメソッド名を含むトレイトを複数実装することも可能。呼び出しの際にどれを呼び出すのかを明示する必要がある。
trait Pilot {
    fn fly(&self) -> &str;
}
trait Wizard {
    fn fly(&self) -> &str;
}
struct Human;

impl Pilot for Human {
    fn fly(&self) -> &str {
        "This is your captain speaking"
    }
}
impl Wizard for Human {
    fn fly(&self) -> &str {
        "Up!"
    }
}
impl Human {
    fn fly(&self) -> &str {
        "*waving arms furiously*"
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        "Spot".to_string()
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        "puppy".to_string()
    }
}

fn full_path() {
    assert_eq!(Dog::baby_name(), "Spot".to_string());
    // Animal::baby_name() はコンパイルエラー => どのbaby_nameかわからない。 Animalトレイトを実装しているものはDog以外にもあるかもしれない
    assert_eq!(<Dog as Animal>::baby_name(), "puppy".to_string());
}

// これはコンパイルエラー => OutlinePrintは fmt::Displayを実装しているものしか実装できない
// impl OutlinePrint for Point {}

use std::fmt;
trait OutlinePrint: fmt::Display {
    fn output_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// Displayを実装していると OutlinePrintも実装できる
struct Point2 {
    x: i32,
    y: i32,
}

impl fmt::Display for Point2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point2 {}

// ニュータイプパターン
// コンパイル時にWrapperを無視するので、実行効率上のデメリットはない
// ただ、Wrapperが本体のため、メソッドが実装されていない。
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn new_type_pattern() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    assert_eq!("[hello, world]", w.to_string());
}
