fn main() {
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
