pub fn chap19_1() {
    raw_pointer_from_reference();
    raw_address();
    unsafe_function();
    split_at_mut(&mut [1, 2, 3, 4, 5, 6], 3);
    bad_manner();
    call_external();
    mut_static_var();
}

// unsafe
// - 生ポインタを参照外しすること
// - unsafeな関数やメソッドを呼ぶこと
// - 可変で性的な変数にアクセスしたり変更すること
// - unsafeなトレイトを実装すること
//
// unsafeは、借用チェッカーや他のRustの安全性チェックを無効にしないことを理解するのは重要なことです:
// unsafeコードで参照を使用しても、チェックはされます。
// unsafeキーワードにより、これら4つの機能にアクセスできるようになり、 その場合、コンパイラによってこれらのメモリ安全性は確認されないのです。
// unsafeブロック内でも、ある程度の安全性は得られます。

fn raw_pointer_from_reference() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let r3 = &num;

    unsafe {
        println!("r1 is: {}", *r1); // unsafe がないと dereference of raw pointer になる
        println!("r2 is: {}", *r2);
    }
    println!("r3 is: {}", *r3);
}

fn raw_address() {
    let address = 0x012345usize;
    let r = address as *const i32;
    println!("{:?}", r);
    unsafe {
        // unsafe がないと dereference of raw pointer になる
        // 値が入っている保証がない未定義動作
        // println!("{:?}", *r);   　　　これはコンパイルエラーにならないが大抵落ちる
    }
}

unsafe fn dangerous() {} // 関数自体がunsafeになるので、この中ではunsafeなブロックを作る必要はない
fn unsafe_function() {
    unsafe {
        dangerous();
    }
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    // sliceはデータへのポインタとスライスの長さ
    let len = slice.len();

    assert!(mid < len);
    // (&mut slice[..mid], &mut slice[mid..])  borrowが2回走っていることだけがわかるのでコンパイルエラーになる

    let ptr = slice.as_mut_ptr(); // *mut i32
    unsafe {
        use std::slice;
        (
            // 関数slice::from_raw_parts_mutは、unsafe
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}

fn bad_manner() {
    use std::slice;

    let address = 0x012345usize;
    let r = address as *mut i32;
    let slice = unsafe {
        // 10000分のメモリを用意している保証がないため、未定義動作に陥る
        slice::from_raw_parts_mut(r, 10000);
    };
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn call_external() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    // マングル処理を行わないことで、Cから呼び出せる。
    // extern キーワードが必要
    println!("Just called a Rust function from C!");
}

static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn mut_static_var() {
    add_to_count(3);
    unsafe {
        //unsafe がないと use of mutable static is unsafe となってコンパイルエラー
        println!("COUNTER: {}", COUNTER);
        // グローバルにアクセス可能な可変なデータはスレッド間でのデータ競合がないことを保証することが難しいためこのようになる。
    }
}

//1つでもunsafeなメソッドがあるとunsafe traitになる
unsafe trait Foo {}
unsafe impl Foo for i32 {}

// 生ポインタなどのSendやSyncでない型を含む型を実装し、その型をSendやSyncでマークしたいなら、 unsafeを使用しなければなりません。
// コンパイラは、型がスレッド間を安全に送信できたり、 複数のスレッドから安全にアクセスできるという保証を保持しているか確かめられません;
// 故に、そのチェックを手動で行い、 unsafeでそのように示唆する必要があります。
