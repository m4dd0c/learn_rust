struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn area (self: &Self) -> u32 {
        self.height * self.width
    }
    fn can_hold (self: &Self, rect: &Rectangle) -> bool {
        self.height > rect.height && self.width > rect.width
    }
}

fn main() {
    let rect1 = Rectangle{
        height: 30,
        width: 50,
    };
    let rect2 = Rectangle {
        height: 10,
        width: 30,
    };
    let rect3 = Rectangle {
        height: 40,
        width: 70,
    };
    println!("Rect 1 can hold Rect 2: {}", rect1.can_hold(&rect2));
    println!("Rect 1 can hold Rect 3: {}", rect1.can_hold(&rect3));
    println!("Area of Rect 1: {}", rect1.area());
}
