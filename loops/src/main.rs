fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (0..5).rev() {
        println!("the value is: {}", a[number]);
    }

    let s = "string";
    println!("{}", s);
}
