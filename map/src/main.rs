use itertools::Itertools;
use regex::Regex;
use std::{cmp::Ordering, collections::HashMap, io};

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
    map.entry(String::from("key3"))
        .or_insert("value3".to_string());
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

    //まとめ1
    println!("stats1: {:#?}", stats(vec![3, 1, 4, 1, 6])); //3,3,1
    println!("stats2: {:#?}", stats(vec![3, 1, 4, 5, 6, 3])); //3,3,3 / 1,3,3,4,5,6
    println!("stats3: {:#?}", stats(vec![3, 1, 4, 5, 6, 3, 3])); //3,3,3 //1,3,3,3,4,5,6
    println!("stats3: {:#?}", stats(vec![3, 1, 4, 5, 6, 3, 3, 4])); //3,3,3 //1,3,3, 3,4,4,5,6

    //まとめ2
    let words = vec!["first", "apple"];
    words
        .iter()
        .for_each(|w| println!("pig-latin: {}, result: {}", w, pig_latin(w)));

    //まとめ3
    let mut people_in_specific_department: HashMap<String, Vec<String>> = HashMap::new();
    let re = Regex::new(r"Add ([a-zA-Z]+) to ([a-zA-Z]+)").unwrap();
    loop {
        println!("Please input name of person with belonged department or \"q\" if you want to finish inputing.\r\nInputing format is \"Add <person name> to <department name>\"");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        match &*input {
            "q" => {
                println!("q is inputed.");
                break;
            }
            _ => (),
        }

        let caps = re.captures(&input).unwrap();
        let person_name = caps
            .get(1)
            .map(|m| m.as_str().to_string())
            .expect("Invalid Input; can't capture name of person.");
        let department_name = caps
            .get(2)
            .map(|m| m.as_str().to_string())
            .expect("Invalid Input; can't capture name of department_name.");

        println!(
            "You inputed that person is {} and department is {}",
            &person_name, &department_name
        );
        let people = people_in_specific_department
            .entry(department_name.clone())
            .or_insert(vec![] as Vec<String>);
        let len = people.len();
        for (idx, name) in people.clone().iter().enumerate() {
            match name.cmp(&person_name) {
                Ordering::Less => {
                    people.insert(idx, person_name.clone());
                    break;
                }
                Ordering::Equal => {
                    people.insert(idx, person_name.clone());
                    break;
                }
                Ordering::Greater => (),
            }
        }
        if people.len() == len {
            people.push(person_name.clone())
        }
    }
    println!(
        "people_in_specific_department: {:#?}",
        people_in_specific_department
    );
}

fn stats(numbers: Vec<i32>) -> (f64, f64, i32) {
    let mean = (numbers.iter().sum::<i32>() as f64) / (numbers.len() as f64);

    let median = {
        let sorted_numbers: Vec<i32> = numbers.clone().into_iter().sorted().collect();
        let indice = [
            (((&sorted_numbers.len() - 1) as f64) / 2.0).ceil() as usize,
            (((&sorted_numbers.len() - 1) as f64) / 2.0).floor() as usize,
        ];
        let sum_of_two_median_values: i32 = indice
            .map(|idx| sorted_numbers.clone().into_iter().nth(idx).unwrap())
            .iter()
            .sum();
        (sum_of_two_median_values as f64) / 2.0
    };

    let most_appeared = {
        let mut freq: HashMap<i32, i32> = HashMap::new();
        for n in numbers.clone() {
            let count = freq.entry(n).or_insert(0);
            *count += 1;
        }
        freq.into_iter()
            .sorted_by(|l, r| Ord::cmp(&l.1, &r.1))
            .last()
            .map(|(k, _)| k)
            .unwrap()
    };
    (mean, median, most_appeared)
}

fn pig_latin(word: &str) -> String {
    let mut vowel = "aiueo".chars();
    let head = word
        .chars()
        .nth(0)
        .expect("empty string is passed to pig_latin as &str");
    match vowel.contains(&head) {
        true => format!("{}-hay", word),
        false => format!("{}-{}ay", &word.to_string()[1..], head), // &strはポイントなので、trait Sized を実装していないためSliceにできないので、to_string()する
    }
}
