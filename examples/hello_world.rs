/*
    Figure out standard library (take lineout out of the .ll file, make the lineout function call (OR ANY SPECIFIED MODULE) a predefined function in the lexer initialization)
*/

fn hello(message: String, number: i32) -> bool {
    println!("{}", message);
    println!("{}", number);
    return true
}

fn main() {
    hello(String::from("hi!"), 2);
}
