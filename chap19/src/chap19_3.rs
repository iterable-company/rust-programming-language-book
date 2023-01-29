pub mod new_type;
pub mod type_alias;

use crate::chap19_3::new_type::{Base, People, Wrapper};
use crate::chap19_3::type_alias::{MyStruct, MyTrait};

pub fn run() {
    new_type_pattern();
    type_alias();
    sized();
}

fn new_type_pattern() {
    let base = Base::new("Taro YAMADA".to_string(), 50);
    assert_eq!(base.name(), "Taro YAMADA".to_string());
    // base.age  => privateフィールドなのでエラー
    let wrapper = Wrapper::new("Taro YAMADA".to_string(), 50);
    assert_eq!(wrapper.name(), "Taro YAMADA".to_string());
    assert_eq!(wrapper.age(), 50);

    let mut people = People::new();
    let shohei_id = people.push("Otani Shohei".to_string());
    let mitoma_id = people.push("Mitoma Kaoru".to_string());
    assert_eq!(people.get(&shohei_id), Some("Otani Shohei".to_string()));
    assert_eq!(people.get(&mitoma_id), Some("Mitoma Kaoru".to_string()));
    assert_eq!(people.get(&3), None);
}

fn type_alias() {
    assert_eq!(<MyStruct as MyTrait>::add_one(1), Ok(2));
    assert_eq!(
        format!("{:?}", <MyStruct as MyTrait>::add_one(0)),
        "Err(MyError { msg: \"argument must be positive value\" })"
    );
}

fn sized() {
    generic1(1);
    // generic1("donpappa"[1..]); これは暗黙のSized つまり generic<T: Sized>となっていることに対して、渡した値がSizedでないのでコンパイルエラーになる
    generic2(&"hogefuga"[1..]);
}

fn generic1<T: std::fmt::Debug>(t: T) {
    println!("{:?}", t);
}
// ?SizedとするとSizeの制限を緩めて、コンパイル時にサイズが決まらなくても良くなる
fn generic2<T: ?Sized + std::fmt::Debug>(t: &T) {
    println!("{:?}", t);
}
