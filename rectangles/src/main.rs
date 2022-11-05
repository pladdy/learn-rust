#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.height > rect.height && self.width > rect.width
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 40,
        height: 30,
    };

    // println!("rect 1 is {:#?}", rect1); <- this is not great
    dbg!(&rect1); // this is better

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!("Can rect2 fit in rect1?: {}", rect1.can_hold(&rect2));
    println!("Can rect3 fit in rect1?: {}", rect1.can_hold(&rect3));
}
