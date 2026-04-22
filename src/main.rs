use std::io;

fn main() {
    // asking for the number of elems from user
    println!("Enter number of elements you need");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Expected string");
    let num: u32 = num.trim().parse().expect("Not a Number");
    println!("Printing {num} Elements of fibonacci series...");
    fibonacci(num);
    println!("Program ended");
}

fn fibonacci(num: u32){
    let mut n1 = 0;
    let mut n2 = 1;
    println!("{n1}");
    println!("{n2}");
    for _i in 1..num-1 {
        let n3 = n1+n2;
        println!("{n3}");
        n1 = n2;
        n2 = n3;
    }
}
