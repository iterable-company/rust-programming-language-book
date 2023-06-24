fn main() {
    demo();
    explicit_drop_coercion();
    explicit_drop_coercion_with_mem_drop();
    use_after_mem_drop();
}

struct CustomSmartPointer {
    pub data: String,
}

// Drop は初期化処理に含まれるので、use でインポートする必要がない
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}!`", self.data);
    }
}

fn demo() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}

fn explicit_drop_coercion() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("CustomSmartPointers created.");
    // c.drop(); // ここでcompile error になる。cがスコープを抜けた時にdropが走るので、二重解放エラーになるため
    println!("CustomSmartPointers dropped before the end of main.");
}

fn explicit_drop_coercion_with_mem_drop() {
    use std::mem::drop;
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("CustomSmartPointers created.");
    drop(c);
    println!("CustomSmartPointers dropped before the end of main.");
}

fn use_after_mem_drop() {
    use std::mem::drop;
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("CustomSmartPointers created.");
    drop(c); // ここで move が発生

    //println!("{} is still usable?", c.data); compile error! --  move後なので使えない
}
