fn main() {
    variable();

    println!("Hello, world!");

    another_function(five(), 3.2);
    take_function(say_good_bye);

    control_flow();
    go_out_loop();
    while_loop();
    for_loop();
}

use std::io;
fn variable() {
    let x = (500, 6.4, 1);
    let f = x.0;
    println!("f = {}", f);
    let f: String = f.to_string();
    println!("f = {}", f);

    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    println!("first = {}", first);

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index.trim().parse().expect("not a number!");
    println!("bomb = {}", a[index]);
}

fn another_function(x: i32, y: f32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn take_function<F>(func: F)
where
    F: Fn() -> (),
{
    println!("hello");
    func();
}

fn say_good_bye() {
    println!("good bye");
}

fn control_flow() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number)
}

fn go_out_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    // 発射！
    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (0..5).rev() {
        println!("the value is: {}", a[number]);
    }

    for i in (10..=20) {
        println!("the value is {}", i);
    }

    let s = "string";
    println!("{}", s);
}
