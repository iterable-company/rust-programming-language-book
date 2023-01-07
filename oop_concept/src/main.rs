fn main() {
    average();
    trait_object();
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
