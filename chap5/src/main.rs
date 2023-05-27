fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{:#?}", rect,);

    println!("{}", area(&rect),);

    println!("The area of the rectangle is {} square pixels", rect.area(),);

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("{}", rect.can_hold(&rect2));
    println!("{}", rect.can_hold(&rect3));
    println!("{:#?}", Rectangle::square(rect.height))
}

fn area(rect: &Rectangle) -> u32 {
    rect.height * rect.width
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width <= self.width && other.height <= self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
