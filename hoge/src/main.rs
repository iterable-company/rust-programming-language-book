use derive_new::new;

#[derive(new, Debug, Clone)]
struct Hoge {
    id: i32,
    name: String,
}

fn main() {
    let a = vec![
        Hoge::new(3, "three".to_string()),
        Hoge::new(4, "four".to_string()),
        Hoge::new(1, "one".to_string()),
    ];
    let b = vec![
        Hoge::new(4, "four2".to_string()),
        Hoge::new(6, "six".to_string()),
        Hoge::new(0, "zero".to_string()),
    ];
    println!("{:?}", extract(a, b));
}

fn extract(a: Vec<Hoge>, b: Vec<Hoge>) -> (Vec<Hoge>, Vec<Hoge>, Vec<Hoge>) {
    let a_ids: Vec<i32> = a.clone().into_iter().map(|a_tmp| a_tmp.id).collect();
    let b_ids: Vec<i32> = b.clone().into_iter().map(|b_tmp| b_tmp.id).collect();

    return (
        a.clone()
            .into_iter()
            .filter(|a_tmp| !b_ids.contains(&a_tmp.id))
            .collect(),
        a.clone()
            .into_iter()
            .filter(|a_tmp| b_ids.contains(&a_tmp.id))
            .collect(),
        b.clone()
            .into_iter()
            .filter(|b_tmp| !a_ids.contains(&b_tmp.id))
            .collect(),
    );
}
