fn main() {
    average();
    trait_object();
    blog();
}

fn average() {
    let v = vec![1, 2, 3, 4];
    let ac = AverageCollection::from_vec(v);
    println!("{}", ac.average());
}

pub struct AverageCollection {
    list: Vec<i32>,
    average: f64,
}

impl AverageCollection {
    pub fn from_vec(list: Vec<i32>) -> Self {
        let average = AverageCollection::calc_average(&list);
        Self {
            list: list,
            average: average,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        self.average = AverageCollection::calc_average(&self.list);
    }

    fn calc_average(list: &Vec<i32>) -> f64 {
        let total: i32 = list.iter().sum();
        total as f64 / list.len() as f64
    }
}

fn trait_object() {
    let button = Button {};
    let select_box = SelectBox { items: Vec::new() };
    let mut trait_objecgt = ScreenTraitObject { components: vec![] };
    trait_objecgt.components.push(Box::new(button.clone()));
    trait_objecgt.components.push(Box::new(select_box.clone()));

    // 以下は ScreenGenericを生成するときに型引数を指定しなくてはいけないため、異なる型を指定できない
    let mut trait_generic = ScreenGeneric {
        components: Vec::<Button>::new(),
    };
}

pub trait Draw {
    fn draw(&self);
}

pub struct ScreenTraitObject {
    pub components: Vec<Box<dyn Draw>>,
}

impl ScreenTraitObject {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct ScreenGeneric<T: Draw> {
    pub components: Vec<T>,
}

impl<T> ScreenGeneric<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Clone)]
struct Button {}
impl Draw for Button {
    fn draw(&self) {
        todo!()
    }
}
#[derive(Clone)]
struct SelectBox {
    pub items: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        todo!()
    }
}

fn blog() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.rquest_review();
    post.add_text("added after review requested");
    assert_eq!("", post.content());

    post.reject();
    post.add_text(" added after review rejected");
    post.rquest_review();

    post.approve();
    assert_eq!(
        "I ate a salad for lunch today added after review rejected",
        post.content()
    );
}

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if self.state.as_ref().unwrap().can_add_text() {
            self.content.push_str(text);
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn rquest_review(&mut self) {
        // take()はself.stateから所有権を奪い代わりにNoneにする
        if let Some(s) = self.state.take() {
            self.state = Some(s.rquest_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    fn rquest_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&'a self, _: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn can_add_text(&self) -> bool {
        false
    }
}

struct Draft {}

struct PendingReview {}

struct Published {}

impl State for Draft {
    fn rquest_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn can_add_text(&self) -> bool {
        true
    }
}

impl State for PendingReview {
    fn rquest_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

impl State for Published {
    fn rquest_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&'a self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
