use rand::Rng;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn add_random(x: i32) -> i32 {
    let mut rng = rand::thread_rng();
    x + rng.gen::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}
