use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let teams = vec![String::from("Bule"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }

    let mut map: HashMap<String, String> = HashMap::new();
    let key = String::from("key");
    let value = String::from("value");
    let b = &map.insert(key, value);
    println!("b: {:#?}", b);
    //println!("{}", key); keyが上にあるinsertでmoveされているので、ここでは使えない
    for (k, v) in &map {
        println!("{}: {}", k, v);
    }

    let score = &scores.get(&"Bule".to_string());
    println!("{:#?}", score);

    let b = &map.insert("key".to_string(), "hoge".to_string());
    println!("b: {:#?}", b);
    map.insert("key2".to_string(), "value2".to_string());
    for (k, v) in &map {
        println!("{}: {}", k, v);
    }

    map.entry(String::from("key")).or_insert("fuga".to_string());
    //&map.entry(String::from("key2")).or_insert("foo".to_string()); 上記のinsertの場合は何も言われないが、&を先頭につけて借用にすると、返した値を変数として宣言するように警告が出る。要らない場合は let _ = のようにする
    map.entry(String::from("key3")).or_insert("value3".to_string());
    //let or_insert_1 = map.entry(String::from("key")).or_insert("fuga".to_string()); 返した値を変数として格納するとborrowが発生してしまう
    //let or_insert_2 = map.entry(String::from("key3")).or_insert("value3".to_string()); 上記
    //println!("or_insert_1: {:#?}", or_insert_1);  "key"は存在しているため, "fuga"では更新されずに、格納されている値である"hoge"が返る
    //println!("or_insert_2: {:#?}", or_insert_2);  "value3"が返る
    for (k, v) in &map {
        println!("{}: {}", k, v);
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("stats1: {:#?}", stats(vec![3, 1, 4, 1, 6]));//3,3,1
    println!("stats2: {:#?}", stats(vec![3, 1, 4, 5, 6, 3]));//3,3,3 / 1,3,3,4,5,6
    println!("stats3: {:#?}", stats(vec![3, 1, 4, 5, 6, 3, 3]));//3,3,3 //1,3,3,3,4,5,6
    println!("stats3: {:#?}", stats(vec![3, 1, 4, 5, 6, 3, 3, 4]));//3,3,3 //1,3,3, 3,4,4,5,6
}

fn stats(numbers: Vec<i32>) -> (f64, f64, i32) {
    let mean = (numbers.iter().sum::<i32>() as f64)/(numbers.len() as f64);
    let sorted_numbers: Vec<i32> = numbers.clone().into_iter().sorted().collect();
    let (smaller_idx, bigger_idx) = ((((sorted_numbers.len() - 1) as f64)/2.0 ).ceil() as i32, (((sorted_numbers.len() - 1) as f64)/2.0 ).floor() as i32);
    println!("(smaller_idx, bigger_idx): {:#?}", (smaller_idx, bigger_idx));
    let picked_numbers_for_median: Vec<i32> = sorted_numbers.iter().enumerate().filter(|(idx, _)| *idx as i32 == smaller_idx || *idx as i32 == bigger_idx).map(|(_, v)| *v).collect();
    println!("picked_numbers_for_median: {:#?}", picked_numbers_for_median);
    let median = (picked_numbers_for_median.iter().sum::<i32>() as f64)/(picked_numbers_for_median.len() as f64);
    let mut freq: HashMap<i32, i32> = HashMap::new();
    for n in numbers.clone() {
        let count = freq.entry(n).or_insert(0);
        *count += 1;
    }
    let most_appeared = freq.into_iter().sorted_by(|l, r| Ord::cmp(&l.1, &r.1)).last().map(|(k, v)| k).unwrap();
    (mean, median, most_appeared)
}

