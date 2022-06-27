fn main() {
    success1();
    success2();
}

fn success1() {
    let mut s = String::from("hello1");
    change(&mut s);

    println!("{}", s);

    s.push_str(", kamigyo");

    println!("{}", s);
}

fn success2() {
    let s = &mut String::from("hello1");
    change(s);

    println!("{}", s);

    s.push_str(", kamigyo");

    println!("{}", s);
}

// fn compile_error1() {
//     let s = mut String::from("hello1");//そもそもこの型宣言ができない
//     change(&s);

//     println!("{}", s);

//     s.push_str(", kamigyo");

//     println!("{}", s);
// }

// fn compile_error2() {
//     let s = String::from("hello1");
//     change(&mut s);//mutableで宣言されていない変数はmutableとして借用できない
// }

// fn compile_error3() {
//     let s = String::from("hello1");
//     s.push_str("string");
//     println!("{}", s);
// }

fn change(str:  & mut String) {
    str.push_str(", kyoto");
}
