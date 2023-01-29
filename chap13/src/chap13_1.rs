pub fn run() {
    move_closure();
    not_move_and_mutate();
    not_move_and_not_mutate();
}

// ref: https://qiita.com/shortheron/items/c1735dc4c7c78b0b55e9
fn call_fn_once<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn call_fn_mut<F>(mut f: F)
where
    F: FnMut(),
{
    f();
}

fn call_fn<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn move_closure() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    let v3 = vec![1, 2, 3];

    call_fn_once(|| {
        println!("{:?}", v1.into_iter().map(|i| i + 1).collect::<Vec<i32>>());
    });

    // v2をmoveしているのでコンパイルエラー
    // call_fn_mut(|| {
    //     println!("{:?}", v2.into_iter().map(|i| i + 1).collect::<Vec<i32>>());
    // });

    // v3をmoveしているのでコンパイルエラー
    // call_fn(|| {
    //     println!("{:?}", v3.into_iter().map(|i| i + 1).collect::<Vec<i32>>());
    // });

    // call_fn_onceでmoveしているのでv1は参照できない
    // println!("{:?}", v1);
}

fn not_move_and_mutate() {
    let mut v1 = vec![1, 2, 3];
    let mut v2 = vec![1, 2, 3];
    let mut v3 = vec![1, 2, 3];

    call_fn_once(|| {
        v1.sort_by(|l, r| l.cmp(&r));
    });

    call_fn_mut(|| {
        v2.sort_by(|l, r| l.cmp(&r));
    });

    // キャプチャしたものを変更しているのでコンパイルエラー
    // call_fn(|| {
    //     v3.sort_by(|l, r| l.cmp(&r));
    // });
}

fn not_move_and_not_mutate() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];
    let v3 = vec![1, 2, 3];

    call_fn_once(|| {
        println!("{:?}", v1);
    });

    call_fn_mut(|| {
        println!("{:?}", v2);
    });

    call_fn(|| {
        println!("{:?}", v3);
    });
}
