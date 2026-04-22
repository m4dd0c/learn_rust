use std::io;

fn main() {
    // asking for the number of elems from user
    println!("Enter number of elements you need");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Expected string");
    println!("num is {num}");
}
