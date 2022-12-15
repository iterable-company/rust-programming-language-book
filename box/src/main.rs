fn main() {
    read();
    cons_list();
}

fn read() {
    let b = Box::new(5); // ヒープに5を確保
    println!("b = {}", b); // スタックにあるのと同じ方法でアクセスできる
}
// スコープを抜けると、スタックに確保されている Box と、ヒープに確保されている値（5）が解放される

// ----- cons ------
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>), // Boxはポインタなのでコンパイル時にサイズが確定する -> List型はi32とBox分のサイズがあれば良いことになる
    Nil,
}

fn cons_list() {
    use List::{Cons, Nil};
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);
}
