fn main() {
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