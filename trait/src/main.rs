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
    fn tick<U: NumCast + NumOps + std::cmp::PartialOrd + std::ops::AddAssign + Clone>(
        self,
        upper_limit: Point2<U>,
        tick: Point2<U>,
    ) -> anyhow::Result<Vec<Point2<U>>> {
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

fn main() {
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
