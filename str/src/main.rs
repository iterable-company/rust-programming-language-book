fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!;");
    let s3 = s1.clone() + &s2;
    let s4 = format!("{}{}", s1, s2);
    println!("{}", s1);
    println!("{}", s3);
    println!("{}", s4);

    //println!("{}", &s1[0]); Stringは添字アクセスを実装していない

    let kyoto_prefecture = String::from("京都府");
    for ch in kyoto_prefecture.chars() {
        println!("{}", ch);
    }
    let hokke = String::from("𩸽の塩焼き");
    for ch in hokke.chars() {
        println!("{}", ch);
    }

    for ch in "नमस्ते".chars() {
        println!("{}", ch);
    }

    for ch in "🙇𠀋".chars() {
        println!("{}", ch);
    }
}
