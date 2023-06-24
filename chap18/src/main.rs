fn main() {
    while_let();
    let_();
    function_args();
    irrefutable(Some(3));
    shadow();
    or();
    range_();
    destructure();
    enum_();
    reference();
    struct_and_tuple();
    suppress_warning();
    ignore_some_part();
    use_ref_not_to_move();
    use_ref_mut();
    match_guard();
    multi_guard();
    at_mark_biding();
}

fn while_let() {
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn let_() {
    let x = 5; // これもパターン！！！
    let (x, y, z) = (1, 2, 3);
}

fn function_args() {
    let point = (3, 3);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn irrefutable(op: Option<i32>) {
    // let Some(x) = op; //op = Noneの時、論駁不可能なため、論駁可能であることが必須な let文では 使えない
}

fn shadow() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn or() {
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn range_() {
    let x = 5;
    match x {
        1..=5 => println!("one through five"),
        _ => println!("something elses"),
    }

    let y = 'c';
    match y {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}
fn destructure() {
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn enum_() {
    enum_match(Message::Quit);
    enum_match(Message::Move { x: 1, y: 0 });
    enum_match(Message::Move { x: 0, y: 1 });
    enum_match(Message::Move { x: 1, y: 2 });
    enum_match(Message::Write("hoge".to_string()));
    enum_match(Message::ChangeColor(0, 165, 255));
}

fn enum_match(message: Message) {
    match message {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x: 0, y } => println!("Move on the x-axis."),
        Message::Move { x, y: 0 } => println!("Move on the y-axis."),
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y)
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}

fn reference() {
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points.iter().map(|&Point { x, y }| x * x + y * y).sum();
}

fn struct_and_tuple() {
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

fn suppress_warning() {
    let s = Some(String::from("Hello!"));

    //if let Some(_s) = s { <- _sでは、警告の抑制にはなるが、moveが起きてしまうためにコンパイルエラー
    if let Some(_) = s {
        // _ ではmoveが起きない
        println!("found a string");
    }

    println!("{:?}", s);
}

struct Point1 {
    x: i32,
    y: i32,
    z: i32,
}
fn ignore_some_part() {
    let origin = Point1 { x: 0, y: 0, z: 0 };

    match origin {
        Point1 { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., fourth, last) => {
            println!("Some numbers: {}, {}, {}", first, fourth, last);
        }
    }
}

fn use_ref_not_to_move() {
    let robot_name = Some(String::from("Bors"));
    match robot_name {
        Some(ref name) => println!("Found a name: {}", name), // refがないとmoveが起きてしまう。また&はパターンマッチに使われるので、参照として取り出すという意味に使えないため、refとなる
        None => (),
    }
    println!("robot_name is: {:?}", robot_name);
}

fn use_ref_mut() {
    let mut robot_name = Some(String::from("Bors"));

    match robot_name {
        Some(ref mut name) => *name = String::from("Another name"),
        None => (),
    }

    println!("robot_name is: {:?}", robot_name);
}

fn match_guard() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

fn multi_guard() {
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"), // (x == 4 or 5 or 6 ) and (y == true)
        _ => println!("no"),
    }
}

enum Message2 {
    Hello { id: i32 },
}
fn at_mark_biding() {
    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello {
            id: id_variable @ 3..=7, // 値を束縛したいので@を使う
        } => {
            println!("Found an id in range: {}", id_variable)
        }
        Message2::Hello { id: 10..=12 } => {
            // 範囲指定したいだけで値は不要なので@は使わない
            println!("Found an id in another range")
        }
        Message2::Hello { id } => {
            println!("Found some other id: {}", id)
        }
    }
}
