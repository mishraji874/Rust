fn main() {
    print!("Hello World"); //but the text is printed to the console
    println!("Hello World"); // for printing in the new line

    format!("Hello world"); //write formatted text to String

    //eprint!: same as print! but the text is printed to the standard error (io::stderr).
    //eprintln!: same as eprint! but a newline is appended.
}