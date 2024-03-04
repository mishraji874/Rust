fn main() {
    let x = 7;

    // example of boolean expression
    let condition = x > 5;

    println!("condition is {}", condition);

    // if condition
    let number = 10;

    // condition to check if number is greater than zero
    if number > 0 {
        println!("{} is greater than 0", number);
    }

    //if-else
    let number = -2;

    // condition to check if number is greater than zero
    if number > 0 {
        println!("{} is greater than 0", number);
    } else {
        println!("{} is less than or equal to 0", number);
    }

    //if-else-if-else

    let number2 = -2;

    if number > 0 {
        println!("{} is positive", number2);
    } else if number < 0 {
        println!("{} is negative", number2);
    } else {
        println!("{} is equal to 0", number2);
    }

    //nested -if

    if number < 0 {
        // if outer condition evaluates to true evaluate the inner condition
        if number == -2 {
            println!("The current number is -2");
        } else {
            println!("The current number is {}", number);
        }
    }
}
