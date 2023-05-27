fn main() {
    generics();
    trait_();
    lifetime();
}

use anyhow::anyhow;
use num_traits::cast::cast;
use num_traits::cast::NumCast;
use num_traits::NumOps;
use std::fmt;

#[derive(Debug)]
struct Point1<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point1<T, U> {
    fn mixup<V, W>(self, other: Point1<V, W>) -> Point1<T, W> {
        Point1 {
            x: self.x,
            y: other.y,
        }
    }
}

impl Point1<i32, i32> {
    fn next(self) -> Point1<i32, i32> {
        Point1 {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

struct Point2<T: NumCast> {
    x: T,
    y: T,
}

impl fmt::Debug for Point2<f64> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point2{{ x: {:.1}, y: {:.1} }}", self.x, self.y)
    }
}

impl<T: NumCast> Point2<T> {
    fn tick<U>(self, upper_limit: Point2<U>, tick: Point2<U>) -> anyhow::Result<Vec<Point2<U>>>
    where
        U: NumCast + NumOps + std::cmp::PartialOrd + std::ops::AddAssign + Clone,
    {
        let mut x: U = cast(self.x).ok_or(anyhow!("can not cast to type of tick"))?;
        let mut y: U = cast(self.y).ok_or(anyhow!("can not cast to type of tick"))?;
        if upper_limit.x <= x || upper_limit.y <= y {
            return Err(anyhow!("upper_limit is smaller than self"));
        }

        let mut ticks_x: Vec<U> = Vec::new();
        loop {
            x += tick.x.clone();
            if upper_limit.x < x {
                break;
            }
            ticks_x.push(x.clone());
        }

        let mut ticks_y: Vec<U> = Vec::new();
        loop {
            y += tick.y.clone();
            if upper_limit.y < y {
                break;
            }
            ticks_y.push(y.clone());
        }
        //cartesian product
        let cross = ticks_x.into_iter().flat_map(|x| {
            ticks_y
                .clone()
                .into_iter()
                .map(move |y| (x.clone(), y.clone()))
        });
        Ok(cross
            .map(|(tick_x, tick_y)| Point2 {
                x: tick_x,
                y: tick_y,
            })
            .collect())
    }
}

fn generics() {
    let p1 = Point1 { x: 5, y: 10.4 };
    let p2 = Point1 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let p4 = Point1 { x: 5, y: 4 };
    let p5 = p4.next();
    print!("{:?}", p5);

    // 以下は Point<i32, f64>にnext()が実装されていないためにコンパイルエラーとなる
    // let p6 = Point { x: 5, y: 4.0 };
    // let p7 = p6.next();
    // println!("{:?}", p7);

    let base_point = Point2 { x: 1, y: 2 };
    let upper_limit = Point2 { x: 1.5, y: 3.9 };
    let tick = Point2 { x: 0.1, y: 0.2 };
    let ticks = base_point.tick(upper_limit, tick).unwrap();
    println!("{:?}", ticks);
}

fn trait_() {
    let t = Twitter {
        author: "hoge".to_string(),
    };
    println!("{}", t.summarize());

    let n1 = News {
        publisher: "impress".to_string(),
    };
    print(&n1);

    let n2 = News {
        publisher: "gihyo".to_string(),
    };

    print_both(&n1, &n2);

    let b = Book {
        publisher: "iwanami".to_string(),
    };
    println!("{}", b);

    // print_both(n1, b);    // n1, b はtrait Summaryを実装しているが、型自体は異なるため、コンパイルエラーとなる

    let number_list = vec![34, 50, 25, 99, 65];
    let result1 = largest(&number_list);
    println!("largest returns {}", result1);

    let result2 = largest_ref(&number_list);
    println!("largest_ref returns {}", result2);
}
pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait Media: Summary {
    fn media_type(&self) -> String;
}

impl<T: Media> Summary for T {
    fn summarize(&self) -> String {
        self.media_type() + ": ..."
    }
}

pub struct Twitter {
    pub author: String,
}

impl Media for Twitter {
    fn media_type(&self) -> String {
        "sns".to_string()
    }
}

pub struct News {
    pub publisher: String,
}

impl Summary for News {
    fn summarize(&self) -> String {
        self.publisher.clone() + " published news."
    }
}

pub struct Book {
    pub publisher: String,
}

impl Summary for Book {
    fn summarize(&self) -> String {
        self.publisher.clone() + " published book."
    }
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Book{{{}}}", self.publisher)
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut max_idx: usize = 0;
    for idx in 0..(list.len() - 1) {
        if list[idx] > list[max_idx] {
            max_idx = idx;
        }
    }
    &(list[max_idx])
}

fn print(item: &impl Summary) {
    println!("{} is printed.", item.summarize());
}

fn print_both<T: Summary>(item1: &T, item2: &T) {
    println!("{} then {}", item1.summarize(), item2.summarize());
}

// 以下のように、戻り値の型をtrait境界を指定して実装することはできない
// impl Summaryを戻り値に使えと言われるが、これが使えるのは、返す値が全て同じ型である場合のみ
// fn return_summary<T: Summary>(yes_or_no: bool) -> T {
//     if yes_or_no {
//         Book {
//             publisher: "kodansha".to_string(),
//         }
//     } else {
//         News {
//             publisher: "kyodo".to_string(),
//         }
//     }
// }

// 10.2によると、「デフォルト実装を、そのメソッドをオーバーライドしている実装から呼び出すことはできないことに注意してください。」 らしい

//以下はTwitterにおけるSummaryの実装がMediaの実装とconflictするためコンパイルエラーとなる
// impl Summary for Twitter {
//     fn summarize(&self) -> String {
//         self.author + " tweeted. ..."
//     }
// }

fn lifetime() {
    // logest
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // substr, substring
    let result2: &str;
    let result3: String;
    {
        let n = 2 as usize;
        result2 = substr(string1.as_str(), n);
        result3 = substring(string1.as_str(), n);
    }
    println!("substr is {}", result2);
    println!("substring is {}", result3);

    // struct
    let novel = String::from("Call me Ishmael. Some years ago. I went to Egypt.");
    let excerpts = novel
        .split(".")
        .enumerate()
        .into_iter()
        .map(|(idx, sentence)| Excerpt {
            idx: idx,
            part: sentence,
        })
        .collect::<Vec<Excerpt>>();
    for excerpt in excerpts {
        println!("idx: {}, part: {}", excerpt.idx, excerpt.part);
    }

    // struct フィールドの参照よりもstructのインスタンスの寿命の方が長い
    let mut excerpt1 = Excerpt {
        idx: 1,
        part: "second sentence",
    };
    {
        let str = "changed";
        excerpt1.part = str;
    }
    println!("{:?}", excerpt1); //excerptの方がpartに設定されている参照よりlifetimeが長いけど大丈夫なの？

    // struct フィールドの参照よりもstructのインスタンスの寿命の方が長い - functionを介してみる バージョン
    let mut excerpt2 = Excerpt {
        idx: 1,
        part: "second sentence",
    };
    {
        let str = "changed2";
        excerpt2 = set_part(excerpt2, str);
    }
    println!("{:?}", excerpt2);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn substr<'a>(x: &'a str, n: usize) -> &'a str {
    &x[..n]
}

fn substring(x: &str, n: usize) -> String {
    x[..n].to_string()
}

#[derive(Debug, Clone)]
struct Excerpt<'a> {
    idx: usize,
    part: &'a str,
}

fn set_part<'a>(excerpt: Excerpt<'a>, part: &'a str) -> Excerpt<'a> {
    Excerpt {
        part: part,
        ..excerpt
    }
}
