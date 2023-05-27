fn main() {
    borrow();
    ownership();
    reference();
    slice();
}

fn borrow() {
    let s = String::from("hello");

    let s1 = &s;
    let s2 = &s;

    println!("{}, world!", s1);
    println!("{}, world!", s2);

    // let mut s3 = &s;
    // s3.push_str(", world"); // ここがs3は&での借用だから、s3が参照するデータはmutableとして借用できないというエラーになる
    // println!("{}, world!", s3);
}

fn ownership() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let x = 5;
    {
        let y = x;
        println!("x = {}, y = {}", x, y);
    }
    println!("x = {}", x);

    takes_ownership(s);
    //println!("{}", s);

    makes_copy(x);
    println!("{}", x);

    let s1 = gives_ownership();
    let s2 = takes_and_gives_back(s1);
    println!("{}", s2);

    let (s3, len) = calculate_length(s2);
    println!("{}, {}", s3, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn reference() {
    let mut s1 = String::from("hello");

    let len = calculate_length(&s1);
    println!("{}, {}", s1, len);

    change(&mut s1);
    change(&mut s1);

    println!("{}", s1);
}

fn calculate_length(some_string: &String) -> usize {
    some_string.len()
}

fn change(some_string: &mut String) {
    let s = some_string.clone();
    some_string.push_str(s.as_str());
}

fn slice() {
    let s = String::from("foo bar buzz");
    //let s1 = first_word(&s);
    let s2 = first_word2(s);
    //println!("{}, {}", s, s1);
    println!("{}", s2);

    let a = [1, 2, 3, 4, 5];
    //let b = a[1..3]; //ローカル変数は必ずサイズが確定していないといけないからコンパイルエラー
    let b = &a[1..3]; //借用だといける。。なぜ？
    for item in b.iter() {
        println!("{}", item);
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, item) in bytes.iter().enumerate() {
        if *item == b' ' {
            return &s[..i];
        }
    }

    &s
}

fn first_word2(s: String) -> String {
    let bytes = s.as_bytes();

    for (i, item) in bytes.iter().enumerate() {
        if *item == b' ' {
            return String::from(&s[..i]);
        }
    }
    s
}
