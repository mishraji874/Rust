fn main() {
    let x: i32 = 5; //if any variable is not initialized and used then it'll give error
    let y: i32; //uninitialized but also unused, only a warning

    println!("{}", x);

    // variable to store integer value
    let age = 31;
    println!("Age: {}", age);

    // variable to store floating-point value
    let salary = 342523.23;
    println!("Salary: {}", salary);

    // variable to store string
    let name = "Jackie";
    println!("Name: {}", name);

    //concept of Mutability
    //mutability is used when we have to change the value of the variable in the future so we can declare the variable with the name of mutable

    // declare a mutable variable with value 1
    let mut x = 1;
    println!("Value of x = {}", x);

    // change the value of variable x
    x = 2;
    println!("Updated value of x = {}", x);
}
