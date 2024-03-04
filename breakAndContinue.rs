fn main() {
    let mut number = 0;
    loop {
        number += 1;
        if number > 5 {
            break;
        }
        println!("{}", number);
    }


    let mut number2 = 0;
    while number2 < 5 {
        number2 += 1;
        if number2 == 3 {
            continue;
        }
        println!("{}", number2);
    }
}