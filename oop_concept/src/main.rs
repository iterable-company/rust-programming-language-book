fn main() {
    average();
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
