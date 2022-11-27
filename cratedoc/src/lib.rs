//! このクレートはドキュメンテーション機能を試すためのものです。
/// 与えられた数値に1を足す
///
/// # Examples
/// ```
/// let five = 5;
/// assert_eq!(6, cratedoc::add_one(five));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
