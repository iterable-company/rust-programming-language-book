fn main() {
    single_thread_mutex();
    rc();
    arc();
}

fn single_thread_mutex() {
    use std::sync::Mutex;

    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap(); // MutexGuardというスマートポインタ型を返す
        *num = 6; // MutexGuardはDeref, DerefMutを実装していて、可変参照としても使える
    }
    println!("m = {:?}", m);
} // Dropでunlockが実装されている

fn rc() {
    use std::rc::Rc;
    use std::sync::Mutex;
    use std::thread;

    let counter = Rc::new(Mutex::new(0));
    let mut handles: Vec<thread::JoinHandle<()>> = vec![];

    /* コンパイルエラーになる Rcはスレッドセーフではないため、スレッド間で共有 きょうゆうできない
    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }
    */

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

fn arc() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
