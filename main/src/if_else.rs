fn main() {
    let number: i32 = 2;
    let number1: i32 = 3;

    // basic if else lol
    if number % 2 == 0 {
        println!("even");
    } else {
        println!("odd");
    }

    // ternary
    let check: &str = if number1 % 2 == 0 {"even"} else {"odd"};
    println!("{}", check);
}