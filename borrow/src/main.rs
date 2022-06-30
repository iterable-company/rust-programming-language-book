fn main() {
    let s = String::from("hello");

    let s1 = &s;
    let s2 = &s;

    println!("{}, world!", s1);
    println!("{}, world!", s2);

    // let mut s3 = &s;
    // s3.push_str(", world"); // ここがs3は&での借用だから、s3が参照するデータはmutableとして借用できないというエラーになる
    // println!("{}, world!", s3);
}
