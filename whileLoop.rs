fn main() {
    let mut counter = 1;
    while counter < 5 {
        println!("{}", counter);
        counter += 1;
    }

    //multipilcation table
    let i = 2;
    let mut j = 1;
    while j <= 10 {
        let mult = i * j;
        println!("{} * {} = {}", i, j, mult);
        j += 1;
    }


}