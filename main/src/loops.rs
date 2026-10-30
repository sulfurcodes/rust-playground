fn main() {
    // for loop 1 to n-1
    for i in 1..6 {
        println!("{}", i);
    }
    println!("--------------");
    // normal loop
    let mut counter: i32 = 1;
    loop {
        if counter == 6 {
            break;
        } else {
            println!("{}", counter);
            counter += 1;
        }
    }
    println!("--------------");
    // while loop
    let mut num: i32 = 1;
    loop {
        while num <= 5 {
            println!("{}", num);
            num += 1;
        } break;
    }
}