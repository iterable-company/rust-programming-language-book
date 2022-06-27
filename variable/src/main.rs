use std::io;

fn main() {
    let x = (500, 6.4, 1);
    let f = x.0;
    println!("f = {}", f);
    let f:String = f.to_string();
    println!("f = {}", f);

    let a = [1,2,3,4,5];
    let first = a[0];
    println!("first = {}", first);

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index:usize = index
        .trim()
        .parse()
        .expect("not a number!");
    println!("bomb = {}", a[index]);
}
