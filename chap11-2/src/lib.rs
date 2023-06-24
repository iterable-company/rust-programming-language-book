pub mod nocapture;
pub mod adder;

#[cfg(test)]
mod tests {

    fn err_result_method() -> Result<i32, String>{
        Err(String::from("error is occured!"))
    }

    #[test]
    fn it_works_result() -> Result<(), String> {
       //err_result_method()?;
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn it_works() {
        //err_result_method().unwrap(); ここで失敗したものがメッセージでわかる
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        };
    }

    #[test]
    fn add_two_works() {
        assert_eq!(4, crate::adder::add_two(2));
    }
}
// 本によると、testで返す型をResultにするとテストコード内で?を使うことができるようになるので便利とあるが、テスト失敗メッセージがわかりにくくなるだけ
// unwrapしてそこで失敗してもらった方が嬉しい -> どこの取り出しで失敗したかがわかる

// test実行方法
// 
// cargo test -- --test-threads=1    スレッド数を指定して実行
// cargo test -- --nocapture         テスト成功時の出力も出力する
// cargo test <string>               <string>をメソッド名に含むテストのみを実行