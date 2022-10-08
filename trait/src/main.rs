use std::fmt;

fn main() {
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
