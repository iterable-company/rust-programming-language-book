fn main() {
    let s = String::from("foo bar buzz");
    //let s1 = first_word(&s);
    let s2 = first_word2(s);
    //println!("{}, {}", s, s1);
    println!("{}", s2);

    let a = [1,2,3,4,5];
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
            return &s[..i]
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
