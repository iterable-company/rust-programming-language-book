use derive_new::new;

#[derive(new)]
pub struct Base {
    name: String,
    age: u32,
}

impl Base {
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct Wrapper(Base);
impl Wrapper {
    pub fn new(name: String, age: u32) -> Self {
        Self {
            0: Base::new(name, age),
        }
    }

    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    pub fn age(&self) -> u32 {
        self.0.age
    }
}

use std::collections::HashMap;
pub struct People(HashMap<i32, String>, i32);

impl People {
    pub fn new() -> Self {
        Self {
            0: HashMap::new(),
            1: 0,
        }
    }

    pub fn push(&mut self, name: String) -> i32 {
        self.1 += 1;
        self.0.insert(self.1, name);
        self.1
    }

    pub fn get(&self, id: &i32) -> Option<String> {
        self.0.get(id).map(|n| n.to_owned())
    }
}
