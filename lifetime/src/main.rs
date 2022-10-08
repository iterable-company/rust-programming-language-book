fn main() {
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
