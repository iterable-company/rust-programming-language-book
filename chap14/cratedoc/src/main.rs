fn main() {
    println!("{}", add_one(3));
}

/// 与えられた数値に1を足す
///
/// # Examples
/// ```
/// let five = 5;
/// assert_eq!(7, createdoc::add_one(five));
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}
