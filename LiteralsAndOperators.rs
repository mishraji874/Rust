fn main() {
    let a = 20;
    let b = 2;

    // add two variables using + operator
    let x = a + b;
    println!("{} + {} = {}", a, b, x);

    // subtract two variables using - operator
    let y = a - b;
    println!("{} - {} = {}", a, b, y);

    // multiply two variables using * operator
    let z = a * b;
    println!("{} * {} = {}", a, b, z);

    let dividend = 21;
    let divisor = 8;

    // arithmetic division using / operator with integers
    let division = dividend / divisor;

    println!("{} / {} = {}", dividend, divisor, division);

    let remainder = dividend % divisor;
    println!("{} % {} = {}", dividend, divisor, remainder);


    //Compound assignmnet operator
    let mut a = 2;
    a += 3;
    println!("a = {}", a);

    //Comparison operators
    let c = 7;
    let d = 3;

    let e = a > b;
    let f = a < b;
    let g = a == b;
    println!("{} >= {} is {}", c, d, e);
    println!("{} <= {} is {}", c, d, f);
    println!("{} == {} is {}", c, d, g);


    //Logical operators
    let a = true;
    let b = false;
    
    // logical AND operation
    let c = a && b;

    // logical OR operation
    let d = a || b;

    // logical NOT operation
    let e = !a;
    
    println!("{} && {} = {}", a, b, c);
    println!("{} || {} = {}", a, b, d);
    println!("!{} = {}", a, e);
}