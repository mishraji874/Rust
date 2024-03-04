fn main() {
    for i in 1..6 {
        println!("{}", i);
    }

    //Sum of First 10 Natural Numbers using for Loop
    let mut sum = 0;
    for i in 1..11 {
        sum += i;
    }
    println!("Sum is {}", sum);
}