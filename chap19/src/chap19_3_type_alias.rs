use derive_new::new;
use std::fmt::Display;

// これをpubにしておかないとMyResultを使うところでエラーになる
#[derive(Debug, PartialEq, new)]
pub struct MyError {
    msg: String,
}
impl Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MyError: {}", self.msg)
    }
}
impl std::error::Error for MyError {}

type MyResult<T> = std::result::Result<T, MyError>;

// 以下はコンパイルエラーになる。 Result型がこのクレートで定義されていないことが原因。エイリアスがまさにエイリアスであることを示している。
// impl<T: Display> Display for MyResult<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Ok(ok) => ok.fmt(f),
//             Err(err) => err.fmt(f),
//         }
//     }
// }
pub trait MyTrait {
    fn add_one(num: i32) -> MyResult<i32> {
        if num > 0 {
            Ok(num + 1)
        } else {
            Err(MyError::new("argument must be positive value".to_string()))
        }
    }
    fn emphasis(word: &str) -> MyResult<String> {
        if word.len() > 0 {
            Ok(word.to_owned() + "!")
        } else {
            Err(MyError::new("argument must not be empty".to_string()))
        }
    }
}
pub struct MyStruct {}
impl MyTrait for MyStruct {}
