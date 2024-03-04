// define a function
fn greet() {
    println!("Hello, World!");
}

fn add() {
    let a = 5;
    let b = 10;

    let sum = a + b;
    println!("Sum of a and b = {}", sum);
}

fn add2(a: i32, b: i32) {
    let sum = a + b;
    println!("Sum of a and b = {}", sum);
}

fn add3(a: i32, b: i32) -> i32 {
    let sum = a + b;
    return sum;
}

fn main() {
    // function call
    greet();
    add();
    add2(2, 11);
    let sum = add3(5, 10);
    println!("Sum of a and b = {}", sum);
}