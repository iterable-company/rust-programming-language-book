pub fn run() {
    let v = vec![1, 2, 3];
    assert_eq!(
        v.into_iter().map(add_one).collect::<Vec<i32>>(),
        vec![2, 3, 4]
    );

    assert_eq!(do_twice(add_one, 1), 4)
}

// クロージャではなくfnだけを受け入れたくなる箇所の一例は、クロージャのない外部コードとのインターフェイスです:
// C関数は引数として関数を受け入れられますが、Cにはクロージャがありません。

fn return_fn_closure1() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
