use std::io;

fn main() {
    fibonacci_til_n();

    println!("Program ended");
}

fn fibonacci_til_n() {
    // asking for the number of elems from user
    println!("Enter number of elements you need: ");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Expected string");
    let num: u32 = num.trim().parse().expect("Only supports numbers above 0.");
    println!("Printing {num} Elements of fibonacci series...");
    fibonacci(num);
}

fn fibonacci(num: u32) {
    let mut n1 = 0;
    let mut n2 = 1;
    if num > 1 {
    println!("{n1}");
    }
    if num > 2 {
    println!("{n2}");
    }
    for _i in 1..num - 1 {
        let n3 = n1 + n2;
        println!("{n3}");
        n1 = n2;
        n2 = n3;
    }
}
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
