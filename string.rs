fn main() {
    let word = String::from("Hello, World!");
    println!("Word = {}", word);


    let mut word = String::from("cat");
    println!("original string = {}", word);
    word.push_str(" dog");
    println!("changed string = {}", word);



    let word = String::from("Hello, World!");

    // slicing a string
    let slice = &word[0..5];

    println!("string = {}", word);
    println!("slice = {}", slice);


    let str = String::from("Hello");
    
    // Loop through each character in a string using chars() method
    for char in str.chars() {
        println!("{}", char);
    }


    let mut word = String::new();
    
    println!("original string = {}", word);
    
    // append a string to the word variable
    word.push_str("Hello, World!");

    println!("changed string = {}", word);
}