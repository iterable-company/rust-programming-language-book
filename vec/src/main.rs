fn main() {
    let v = vec![1,2,3];// default は i32
    println!("{:#?}", v);

    //println!("{}", v[4]); panic
    println!("{:#?}", v.get(4)); // return Option<i32>

    for i in v {
        println!("{}", i);
    }

    let mut v2 = Vec::new();
    v2.push(5);
    println!("{:#?}", v2);

    let mut v3 = vec![10, 9, 8, 7, 6];
    for i in &mut v3 {
        *i += 50;
    }
    println!("{:#?}", v3);

    let v4 = vec![
        SpreadSheetCell::Int(8),
        SpreadSheetCell::Float(4.4),
        SpreadSheetCell::Text("hoge".to_string()),
    ];

    for e in v4 {
        println!("{:#?}",e);
    }
}

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}