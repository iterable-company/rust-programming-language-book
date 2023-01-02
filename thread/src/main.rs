fn main() {
    spawn(2);
    spawn(1);
    join_handle();
    join_handle_before_second_loop_exec();
    mut_after_move();
}

fn spawn(main_thread_sleep_millis: u64) {
    use std::thread;
    use std::time::Duration;

    thread::spawn(move || {
        for i in 1..10 {
            println!(
                "{}: hi number {} from the spawn thread!",
                main_thread_sleep_millis, i
            );
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!(
            "{}: hi number {} from the main thread!",
            main_thread_sleep_millis, // move後に使用している！ Copy trait を実装しているものは問題ない？ メモリ構造を見れば、スタック領域が確保されているかどうかで分かりそう。
            i
        );
        thread::sleep(Duration::from_millis(main_thread_sleep_millis));
    }
}

fn join_handle() {
    use std::thread;
    use std::time::Duration;

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("join: hi number {} from the spawn thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("join: hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn join_handle_before_second_loop_exec() {
    use std::thread;
    use std::time::Duration;

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("join2: hi number {} from the spawn thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("join2: hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn mut_after_move() {
    use std::thread;
    use std::time::Duration;

    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    thread::spawn(move || {
        for i in v {
            println!("hi number {} from the spawn thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    //v.remove(8);      move後に使用しているのでコンパイルエラー
    //v.remove(9);      move後に使用しているのでコンパイルエラー
    //for i in v {      move後に使用しているのでコンパイルエラー
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
