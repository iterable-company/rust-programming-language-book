fn main() {
  let mut us: usize = 0;    // usizeは負の値を割り当てられない
  us -= 1;                  // usizeが引き算して負の値になるときは、panic が起きる
  println!("{}", us); 
}
